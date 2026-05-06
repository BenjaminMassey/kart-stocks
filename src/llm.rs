pub fn init() -> llamacpp_embed::LlamaEmbedModel {
    llamacpp_embed::LlamaEmbedBuilder::new("./llama-model/model.gguf")
        .with_mmproj("./llama-model/mmproj.gguf")
        .with_system_prompt("You are an image identifying robot.")
        .with_parallel(2)
        .with_context_size(16384)
        .build()
        .unwrap()
}

pub fn get_placement_data() -> Vec<llamacpp_embed::VisionMessage> {
    get_training_data("./data/images/places/", &placement_prompt())
}

pub fn get_item_data() -> Vec<llamacpp_embed::VisionMessage> {
    get_training_data("./data/images/items/", &item_prompt())
} // TODO: missing some items

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
    "What placement number is this? Reply only with the placement number itself. If there appears to be no number, reply \"none\"".to_owned()
}

pub fn item_prompt() -> String {
    format!(
        "These are the items in Mario Kart World: {}. Which item is this image depicting? If it does not appear to be an item - mainly a black space - then reply: none. Reply only with the item name itself.",
        crate::data::get_items().join(", "),
    )
}

pub fn identify(
    model: &mut llamacpp_embed::LlamaEmbedModel,
    prompt: &str,
    image_bytes: &[u8],
    training_data: &[llamacpp_embed::VisionMessage],
    id_slot: u64,
) -> String {
    let chat = llamacpp_embed::chat_with_image_bytes(
        model,
        prompt,
        image_bytes,
        "image/jpeg",
        Some(training_data),
        Some(id_slot),
    );
    if let Ok(message) = chat {
        return clean_response(&message.response);
    } else if let Err(message) = chat {
        eprintln!("LLM error: \"{:?}\".", message);
    }
    "FAILURE".to_owned()
}

fn clean_response(response: &str) -> String {
    response
        .trim()
        .replace("\n", "")
        .replace(" ", "-")
        .replace("_", "-")
        .replace("/", "")
        .replace("</think>", "")
        .replace(".", "")
        .replace(",", "")
}
