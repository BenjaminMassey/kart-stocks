use base64::Engine;
use obws::requests::sources::{SourceId, TakeScreenshot};

pub fn choose_obs_source(settings: &crate::settings::Settings) -> uuid::Uuid {
    let sources: Vec<(uuid::Uuid, String)> = get_obs_sources(settings);
    println!("Here are your options:");
    for (i, (_uuid, name)) in sources.iter().enumerate() {
        println!("  {}: \"{}\"", i, name);
    }
    let choice = prompted::input!("\nSource index: ")
        .parse::<usize>()
        .unwrap();
    sources[choice].0
}

fn get_obs_sources(settings: &crate::settings::Settings) -> Vec<(uuid::Uuid, String)> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let obs_sources = rt.block_on(async {
        let client = obws::Client::connect(
            &settings.obs.ip,
            settings.obs.port,
            Some(settings.obs.password.clone()),
        )
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
    settings: &crate::settings::Settings,
    source: uuid::Uuid,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = obws::Client::connect(
            &settings.obs.ip,
            settings.obs.port,
            Some(settings.obs.password.clone()),
        )
        .await
        .unwrap();

        let screenshot: String = client
            .sources()
            .take_screenshot(TakeScreenshot {
                source: SourceId::Uuid(source),
                format: "png",
                width: Some(crate::RESOLUTION.0 as u32),
                height: Some(crate::RESOLUTION.1 as u32),
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
