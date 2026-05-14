use std::sync::{Arc, Mutex};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

pub struct InvestmentAction {
    pub is_buy: bool, // as opposed to a sell
    pub value: i32,
}

#[tokio::main]
pub async fn run(
    settings: &crate::settings::Settings,
    state: Arc<Mutex<crate::data::State>>,
    db_conn: Arc<Mutex<rusqlite::Connection>>,
    mut receive_from_hotkey: tokio::sync::mpsc::UnboundedReceiver<String>,
    token: &str,
    send_to_window: tokio::sync::mpsc::UnboundedSender<InvestmentAction>,
) {
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        StaticLoginCredentials,
    >::new(ClientConfig::new_simple(
        StaticLoginCredentials::new(settings.twitch.bot_channel.clone(), Some(token.to_owned())),
    ));

    let settings_for_mpsc = settings.clone();
    let client_for_mpsc = client.clone();
    let mpsc_thread = tokio::spawn(async move {
        while let Some(hotkey_msg) = receive_from_hotkey.recv().await {
            println!("Hotkey bot trigger: \"{}\".", &hotkey_msg);
            if let Err(e) = client_for_mpsc
                .say(settings_for_mpsc.twitch.bot_channel.clone(), hotkey_msg)
                .await
            {
                eprintln!("twitch client.say() error: {:?}", e);
            }
        }
    });

    let settings_for_chat = settings.clone();
    let state_for_chat = Arc::clone(&state);
    let client_for_chat = client.clone();
    let db_conn_for_chat = db_conn.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "User \"{}\" said \"{}\".",
                        msg.sender.name, msg.message_text,
                    );
                    if let Some(response) = game_interactions(
                        &settings_for_chat.clone(),
                        Arc::clone(&state_for_chat),
                        db_conn_for_chat.clone(),
                        msg,
                        send_to_window.clone(),
                    ) {
                        println!("Bot response: \"{}\".", &response);
                        if let Err(e) = client_for_chat
                            .say(settings_for_chat.twitch.racer_channel.clone(), response)
                            .await
                        {
                            eprintln!("twitch client.say(..) error: {:?}", e);
                        }
                    }
                }
                other => {
                    if settings_for_chat.twitch.debug_messages {
                        println!("[debug] {other:?}");
                    }
                }
            }
        }
    });

    client.join(settings.twitch.racer_channel.clone()).unwrap();

    println!(
        "Joined channel {}: will report messages.",
        &settings.twitch.racer_channel,
    );

    join_handle.await.unwrap();
    mpsc_thread.await.unwrap();

    let mut state_lock = state.lock().unwrap();
    state_lock.running = false;
}

fn game_interactions(
    settings: &crate::settings::Settings,
    state: Arc<Mutex<crate::data::State>>,
    db_conn: Arc<Mutex<rusqlite::Connection>>,
    msg: PrivmsgMessage,
    send_to_window: tokio::sync::mpsc::UnboundedSender<InvestmentAction>,
) -> Option<String> {
    if msg.message_text.to_lowercase() == "!value" {
        return Some(format!("Current value: {}", state.lock().unwrap().value,));
    } else if msg.message_text.to_lowercase() == "!join" {
        match crate::portfolio::add_shareholder(&db_conn, settings, &msg.sender.name) {
            Ok(_) => {
                return Some(format!(
                    "Welcome to the stock market, @{}!",
                    msg.sender.name
                ));
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    } else if msg.message_text.to_lowercase() == "!money" {
        match crate::portfolio::get_shareholder(&db_conn, &msg.sender.name) {
            Ok(shareholder) => {
                return Some(if shareholder.invested {
                    format!(
                        "@{}: you have ${} (invested at ${}).",
                        shareholder.username, shareholder.money, shareholder.price
                    )
                } else {
                    format!(
                        "@{}: you have ${}.",
                        shareholder.username, shareholder.money
                    )
                })
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    } else if msg.message_text.to_lowercase() == "!buy" {
        let current_value = state.lock().unwrap().value;
        match crate::portfolio::invest(&db_conn, &msg.sender.name, current_value) {
            Ok(_) => {
                send_to_window
                    .send(InvestmentAction {
                        is_buy: true,
                        value: current_value,
                    })
                    .unwrap();
                return Some(format!(
                    "@{}: bought in for ${}.",
                    msg.sender.name, current_value
                ));
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    } else if msg.message_text.to_lowercase() == "!sell" {
        let current_value = state.lock().unwrap().value;
        match crate::portfolio::sell(&db_conn, &msg.sender.name, current_value) {
            Ok(_) => {
                let net = current_value
                    - crate::portfolio::get_shareholder(&db_conn, &msg.sender.name)
                        .unwrap()
                        .price;
                send_to_window
                    .send(InvestmentAction {
                        is_buy: false,
                        value: net,
                    })
                    .unwrap();
                return Some(format!(
                    "@{}: sold for ${}.",
                    msg.sender.name, current_value
                ));
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
    None
}
