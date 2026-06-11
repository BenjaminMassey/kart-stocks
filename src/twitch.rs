use std::sync::{Arc, Mutex};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

const INFORMATION_COOLDOWN: u64 = 15;

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
                .say(settings_for_mpsc.twitch.racer_channel.clone(), hotkey_msg)
                .await
            {
                eprintln!("twitch client.say() error: {:?}", e);
            }
        }
    });

    let mut last_info =
        std::time::Instant::now() - std::time::Duration::from_secs(INFORMATION_COOLDOWN);
    let mut last_commands =
        std::time::Instant::now() - std::time::Duration::from_secs(INFORMATION_COOLDOWN);
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
                        &mut last_info,
                        &mut last_commands,
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
    last_info: &mut std::time::Instant,
    last_commands: &mut std::time::Instant,
) -> Option<String> {
    let message = msg.message_text.to_lowercase().trim().to_owned();
    if &message == "!value" {
        return Some(format!("Current value: {}", state.lock().unwrap().value,));
    } else if &message == "!money" {
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
    } else if &message == "!buy" {
        let current_value = state.lock().unwrap().value;
        match crate::portfolio::invest(&db_conn, settings, &msg.sender.name, current_value) {
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
    } else if &message == "!sell" {
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
    } else if &message == "!info" && last_info.elapsed().as_secs() > INFORMATION_COOLDOWN {
        *last_info = std::time::Instant::now();
        return Some(
            "Kart Stocks is an interactive stock-trading game with \
            live Mario Kart World gameplay. Just !buy a share of the \
            current race for the current !value (also on screen) and !sell \
            it later. The value is determined by how good the run is: \
            screenshots are analyzed to determine placement, items, and coin \
            count, which is fed into an equation for a single cost value. Buy \
            low, sell high, and good luck!"
                .to_owned(),
        );
    } else if &message == "!commands" && last_commands.elapsed().as_secs() > INFORMATION_COOLDOWN {
        *last_commands = std::time::Instant::now();
      return Some(
             "!buy: purchase a share at current cost ; \
             !sell: exchange share for current value ; \
             !money: check your current captial potential ; \
             !value: check the current investment cost ; \
             !info: brief overview of game ; \
             !github: code repository link ; \
             !commands: this!"
                .to_owned(),
        );
    } else if &message == "!github" {
        return Some("https://github.com/BenjaminMassey/kart-stocks".to_owned());
    }
    None
}
