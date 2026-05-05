# README

TODO

## OBS Setup

Need to go into OBS, click "Tools" on the top taskbar, click "WebSocket Server Settings", check to enable, then grab the listed password, and place it in `./obws_password.txt`.

## Testing

Use `cargo test -- --nocapture` to see the `println!(..)`s.

## Model

Using Qwen3-VL-8B-Instruct with Q4_K_M (plus f16 mmproj) from here: https://huggingface.co/Qwen/Qwen3-VL-8B-Instruct-GGUF
