use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use std::sync::{Arc, Mutex};

const CHANNEL_NAME: &str = "beanssbm";

// TODO: messages *from* bot
// TODO: actual stock game real stuff

#[tokio::main]
pub async fn run(state: Arc<Mutex<crate::data::State>>) {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let state_for_tokio = Arc::clone(&state);
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
                        println!(
                            "Twitch-side value: {}",
                            state_for_tokio.lock().unwrap().value
                        );
                    }
                }
                _ => {}
            }
        }
    });

    client.join(CHANNEL_NAME.to_owned()).unwrap();

    println!("Joined channel {CHANNEL_NAME}: will report messages.");

    join_handle.await.unwrap();

    let mut state_lock = state.lock().unwrap();
    state_lock.running = false;
}
