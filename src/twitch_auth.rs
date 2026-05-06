use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";
const REDIRECT_URI: &str = "http://localhost:3000";
const TOKEN_FILE: &str = "token.json";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Serialize, Deserialize)]
struct StoredToken {
    token: String,
    expires_in: u64,
    retrieval_time: u64,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn fetch_token() -> String {
    if let Ok(contents) = std::fs::read_to_string(TOKEN_FILE) {
        if let Ok(stored) = serde_json::from_str::<StoredToken>(&contents) {
            if now_secs() < stored.retrieval_time + stored.expires_in {
                println!("Using cached token.");
                return stored.token;
            }
            println!("Cached token expired, re-authorizing.");
        }
    }

    let auth_url = format!(
        "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=chat:read%20chat:edit",
        CLIENT_ID, REDIRECT_URI
    );

    println!("Opening browser to authorize the bot...");
    open::that(&auth_url)
        .unwrap_or_else(|_| println!("Could not open browser. Visit manually:\n{}\n", auth_url));

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let (mut stream, _) = listener.accept().unwrap();

    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf[..n]);

    let first_line = request.lines().next().unwrap().to_owned();

    if let Some(err) = first_line.split("error_description=").nth(1) {
        let msg = err.split(|c| c == '&' || c == ' ').next().unwrap_or(err);
        panic!("Twitch OAuth error: {}", msg.replace('+', " "));
    }

    let code = first_line
        .split("code=")
        .nth(1)
        .expect("no code in redirect")
        .split(|c| c == '&' || c == ' ')
        .next()
        .unwrap()
        .to_owned();

    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 22\r\n\r\nAuthorization complete!")
        .unwrap();

    let response: TokenResponse = reqwest::blocking::Client::new()
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", REDIRECT_URI),
        ])
        .send()
        .unwrap()
        .json()
        .unwrap();

    let stored = StoredToken {
        token: response.access_token.clone(),
        expires_in: response.expires_in,
        retrieval_time: now_secs(),
    };

    std::fs::write(TOKEN_FILE, serde_json::to_string_pretty(&stored).unwrap()).unwrap();
    println!("Token saved to {TOKEN_FILE}.");

    response.access_token
}
