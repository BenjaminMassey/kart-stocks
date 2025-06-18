use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

const CHANNEL_NAME: &str = "beanssbm";

#[tokio::main]
pub async fn run() {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport,StaticLoginCredentials>::new(config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!(
                        "User \"{}\" said \"{}\".",
                        msg.sender.name,
                        msg.message_text,
                    );
                },
                _ => {}
            }
        }
    });

    client.join(CHANNEL_NAME.to_owned()).unwrap();

    println!("Joined channel {CHANNEL_NAME}: will report messages.");

    join_handle.await.unwrap();
}