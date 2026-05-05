const OBWS_PASSWORD_FILE: &str = "./obws_password.txt";

use base64::Engine;
use obws::requests::sources::{SourceId, TakeScreenshot};

pub fn get_obws_password() -> String {
    std::fs::read_to_string(OBWS_PASSWORD_FILE)
        .expect(&format!(
            "Failed to read OBWS password from \"{}\"",
            OBWS_PASSWORD_FILE
        ))
        .trim()
        .to_owned()
}

pub fn choose_obs_source(password: &str) -> uuid::Uuid {
    let sources: Vec<(uuid::Uuid, String)> = get_obs_sources(password);
    println!("Here are your options:");
    for (i, (_uuid, name)) in sources.iter().enumerate() {
        println!("  {}: \"{}\"", i, name);
    }
    let choice = prompted::input!("\nSource index: ")
        .parse::<usize>()
        .unwrap();
    sources[choice].0
}

fn get_obs_sources(password: &str) -> Vec<(uuid::Uuid, String)> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let obs_sources = rt.block_on(async {
        let client = obws::Client::connect("127.0.0.1", 4455, Some(password))
            .await
            .unwrap();
        client.inputs().list(None).await.unwrap()
    });

    obs_sources
        .iter()
        .map(|input| (input.id.uuid, input.id.name.clone()))
        .collect()
}

pub fn get_obs_frame(
    password: &str,
    source: uuid::Uuid,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = obws::Client::connect("127.0.0.1", 4455, Some(password))
            .await
            .unwrap();

        let screenshot: String = client
            .sources()
            .take_screenshot(TakeScreenshot {
                source: SourceId::Uuid(source),
                format: "png",
                width: Some(1280),
                height: Some(720),
                compression_quality: Some(-1), // -1 = default
            })
            .await
            .unwrap();

        let data = screenshot
            .split_once("base64,")
            .map(|(_, data)| data)
            .unwrap();

        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .unwrap();

        image::load_from_memory(&bytes).unwrap().into_rgb8()
    })
}
