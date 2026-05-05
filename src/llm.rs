pub fn init() -> llamacpp_embed::LlamaEmbedModel {
    llamacpp_embed::LlamaEmbedBuilder::new("./llama-model/model.gguf")
        .with_mmproj("./llama-model/model.gguf")
        .with_system_prompt("You are an image identifying robot.")
        .with_parallel(2)
        .with_context_size(8192)
        .build()
        .unwrap()
}

pub fn get_placement_data() -> Vec<llamacpp_embed::VisionMessage> {
    get_training_data("./data/images/place/", &placement_prompt())
}

pub fn get_item_data() -> Vec<llamacpp_embed::VisionMessage> {
    get_training_data("./data/images/first/", &item_prompt())
}

fn get_training_data(dir: &str, prompt: &str) -> Vec<llamacpp_embed::VisionMessage> {
    let mut data: Vec<llamacpp_embed::VisionMessage> = vec![];
    for item in std::fs::read_dir(dir).unwrap() {
        if let Ok(file) = item {
            let path = file.path();
            let name = file
                .file_name()
                .into_string()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>()[0]
                .to_owned();
            data.push(llamacpp_embed::VisionMessage::Multi {
                role: "user".to_string(),
                content: vec![
                    llamacpp_embed::ContentPart::Text {
                        text: prompt.to_owned(),
                    },
                    llamacpp_embed::ContentPart::ImageUrl {
                        image_url: llamacpp_embed::ImageUrl {
                            url: llamacpp_embed::image_path_to_url(&path),
                        },
                    },
                ],
            });
            data.push(llamacpp_embed::VisionMessage::Text {
                role: "assistant".to_string(),
                content: name,
            })
        }
    }
    data
}

pub fn placement_prompt() -> String {
    "What placement number is this? Reply only with the placement number itself.".to_owned()
}

pub fn item_prompt() -> String {
    "These are the items in Mario Kart World: banana, bombomb, boo, boomerang, bullet-bill, coin-block, feather, golden_mushroom, green-shell, horn, lightning, mega-mushroom, mushroom, star, triple-banana, triple-green-shell, triple-mushroom, triple-red-shell. Which item is this image depicting? If it does not appear to be an item - mainly a black space - then reply: none. Reply only with the item name itself.".to_owned()
}

pub fn identify(
    model: &mut llamacpp_embed::LlamaEmbedModel,
    prompt: &str,
    image_bytes: &[u8],
    training_data: &[llamacpp_embed::VisionMessage],
    id_slot: u64,
) -> String {
    clean_response(
        &llamacpp_embed::chat_with_image_bytes(
            model,
            prompt,
            image_bytes,
            "image/jpeg",
            Some(training_data),
            Some(id_slot),
        )
        .unwrap()
        .response,
    )
}

fn clean_response(response: &str) -> String {
    response
        .trim()
        .replace("\n", "")
        .replace(" ", "-")
        .replace("_", "-")
        .replace("/", "")
        .replace("</think>", "")
}
