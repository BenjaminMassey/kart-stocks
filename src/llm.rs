pub fn init() -> llamacpp_embed::LlamaEmbedModel {
    llamacpp_embed::start(
        "./llama-model/model.gguf",
        Some("./llama-model/mmproj.gguf"),
        "You are an image identification robot.",
        60,
        false,
    )
    .unwrap()
}

pub fn get_training_data() -> Vec<llamacpp_embed::VisionMessage> {
    vec![
        llamacpp_embed::VisionMessage::Multi {
            role: "user".to_string(),
            content: vec![
                llamacpp_embed::ContentPart::Text {
                    text: item_prompt(),
                },
                llamacpp_embed::ContentPart::ImageUrl {
                    image_url: llamacpp_embed::ImageUrl {
                        url: llamacpp_embed::image_path_to_url(&std::path::Path::new(
                            "./data/images/first/banana.png",
                        )),
                    },
                },
            ],
        },
        llamacpp_embed::VisionMessage::Multi {
            role: "assistant".to_string(),
            content: vec![llamacpp_embed::ContentPart::Text {
                text: "banana".to_owned(),
            }],
        },
    ]
} // TODO: proper, plus placement / item separation

pub fn placement_prompt() -> String {
    "What placement number is this? Reply only with the placement number itself.".to_owned()
} // TODO: proper

pub fn item_prompt() -> String {
    "What Mario Kart World item is this? Reply only with the item name itself.".to_owned()
} // TODO: proper

pub fn identify(
    model: &mut llamacpp_embed::LlamaEmbedModel,
    prompt: &str,
    image_bytes: &[u8],
    training_data: &[llamacpp_embed::VisionMessage],
) -> String {
    llamacpp_embed::chat_with_image_bytes(
        model,
        prompt,
        image_bytes,
        "image/jpeg",
        Some(training_data),
    )
    .unwrap()
    .response
} // TODO: some parsing after response
