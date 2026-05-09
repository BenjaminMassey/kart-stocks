use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

use std::sync::{Arc, Mutex};

const TWITCH_IRC_DEBUG: bool = false;
const CHANNEL_NAME: &str = "beanssbm";
const BOT_NAME: &str = "kart_stocks";

#[tokio::main]
pub async fn run(
    state: Arc<Mutex<crate::data::State>>,
    mut receive_from_hotkey: tokio::sync::mpsc::UnboundedReceiver<String>,
    token: &str,
) {
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        StaticLoginCredentials,
    >::new(ClientConfig::new_simple(
        StaticLoginCredentials::new(BOT_NAME.to_owned(), Some(token.to_owned())),
    ));

    let client_for_mpsc = client.clone();
    let mpsc_thread = tokio::spawn(async move {
        while let Some(hotkey_msg) = receive_from_hotkey.recv().await {
            println!("Hotkey bot trigger: \"{}\".", &hotkey_msg);
            if let Err(e) = client_for_mpsc
                .say(CHANNEL_NAME.to_owned(), hotkey_msg)
                .await
            {
                eprintln!("twitch client.say() error: {:?}", e);
            }
        }
    });

    let state_for_chat = Arc::clone(&state);
    let client_for_chat = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "User \"{}\" said \"{}\".",
                        msg.sender.name, msg.message_text,
                    );
                    let mut response: Option<String> = None;
                    if msg.message_text.to_lowercase() == "!value" {
                        response = Some(format!(
                            "Current value: {}",
                            state_for_chat.lock().unwrap().value,
                        ));
                    } else if msg.message_text.to_lowercase() == "!join" {
                        match crate::portfolio::add_shareholder(&msg.sender.name) {
                            Ok(_) => {
                                response = Some(format!(
                                    "Welcome to the stock market, @{}!",
                                    msg.sender.name
                                ));
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                            }
                        }
                    } else if msg.message_text.to_lowercase() == "!money" {
                        match crate::portfolio::get_shareholder(&msg.sender.name) {
                            Ok(shareholder) => {
                                response = Some(if shareholder.invested {
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
                        let current_value = state_for_chat.lock().unwrap().value;
                        match crate::portfolio::invest(&msg.sender.name, current_value) {
                            Ok(_) => {
                                response = Some(format!(
                                    "@{}: bought in for ${}.",
                                    msg.sender.name, current_value
                                ))
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                            }
                        }
                    } else if msg.message_text.to_lowercase() == "!sell" {
                        let current_value = state_for_chat.lock().unwrap().value;
                        match crate::portfolio::sell(&msg.sender.name, current_value) {
                            Ok(_) => {
                                response = Some(format!(
                                    "@{}: sold for ${}.",
                                    msg.sender.name, current_value
                                ));
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                            }
                        }
                    }
                    if let Some(res) = response {
                        println!("Bot response: \"{}\".", &res);
                        if let Err(e) = client_for_chat.say(CHANNEL_NAME.to_owned(), res).await {
                            eprintln!("twitch client.say(..) error: {:?}", e);
                        }
                    }
                }
                unknown_msg => {
                    if TWITCH_IRC_DEBUG {
                        println!("[debug] {unknown_msg:?}");
                    }
                }
            }
        }
    });

    client.join(CHANNEL_NAME.to_owned()).unwrap();

    println!("Joined channel {CHANNEL_NAME}: will report messages.");

    join_handle.await.unwrap();
    mpsc_thread.await.unwrap();

    let mut state_lock = state.lock().unwrap();
    state_lock.running = false;
}
