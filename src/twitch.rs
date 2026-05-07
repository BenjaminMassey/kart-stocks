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
pub async fn run(state: Arc<Mutex<crate::data::State>>, token: &str) {
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        StaticLoginCredentials,
    >::new(ClientConfig::new_simple(
        StaticLoginCredentials::new(BOT_NAME.to_owned(), Some(token.to_owned())),
    ));

    let state_for_tokio = Arc::clone(&state);
    let client_for_tokio = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "User \"{}\" said \"{}\".",
                        msg.sender.name, msg.message_text,
                    );
                    if msg.sender.name.to_lowercase() == "beanssbm"
                        && msg.message_text.to_lowercase() == "!stop"
                    {
                        println!("STOPPING!");
                        return;
                    } else if msg.message_text.to_lowercase() == "!value" {
                        client_for_tokio.say(
                            CHANNEL_NAME.to_owned(),
                            format!(
                                "Current value: {}",
                                state_for_tokio.lock().unwrap().value,
                            )
                        ).await.unwrap();
                    } else if msg.message_text.to_lowercase() == "!join" {
                        let response = match crate::portfolio::add_shareholder(&msg.sender.name) {
                            Ok(_) => {
                                format!("Welcome to the stock market, @{}!", msg.sender.name)
                            }
                            Err(e) => {
                                format!("{}", e)
                            }
                        };
                        client_for_tokio
                            .say(CHANNEL_NAME.to_owned(), response)
                            .await
                            .unwrap();
                    } else if msg.message_text.to_lowercase() == "!money" {
                        let response = match crate::portfolio::get_shareholder(&msg.sender.name) {
                            Ok(shareholder) => {
                                if shareholder.invested {
                                    format!(
                                        "@{}: you have ${} (invested at ${}).",
                                        shareholder.username, shareholder.money, shareholder.price
                                    )
                                } else {
                                    format!(
                                        "@{}: you have ${}.",
                                        shareholder.username, shareholder.money
                                    )
                                }
                            }
                            Err(e) => {
                                format!("{}", e)
                            }
                        };
                        client_for_tokio
                            .say(CHANNEL_NAME.to_owned(), response)
                            .await
                            .unwrap();
                    } else if msg.message_text.to_lowercase() == "!buy" {
                        let current_value = state_for_tokio.lock().unwrap().value;
                        let response =
                            match crate::portfolio::invest(&msg.sender.name, current_value) {
                                Ok(_) => {
                                    format!(
                                        "@{}: bought in for ${}.",
                                        msg.sender.name, current_value
                                    )
                                }
                                Err(e) => {
                                    format!("{}", e)
                                }
                            };
                        client_for_tokio
                            .say(CHANNEL_NAME.to_owned(), response)
                            .await
                            .unwrap();
                    } else if msg.message_text.to_lowercase() == "!sell" {
                        let current_value = state_for_tokio.lock().unwrap().value;
                        let response = match crate::portfolio::sell(&msg.sender.name, current_value)
                        {
                            Ok(_) => {
                                format!("@{}: sold for ${}.", msg.sender.name, current_value)
                            }
                            Err(e) => {
                                format!("{}", e)
                            }
                        };
                        client_for_tokio
                            .say(CHANNEL_NAME.to_owned(), response)
                            .await
                            .unwrap();
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

    let mut state_lock = state.lock().unwrap();
    state_lock.running = false;
}
