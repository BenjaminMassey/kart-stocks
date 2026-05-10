# README

TODO

## OBS Setup

Need to go into OBS, click "Tools" on the top taskbar, click "WebSocket Server Settings", check to enable, then grab the listed password, and place it in `./obws_password.txt`.

You will also add the "Kart Stocks" (default name) window. If OBS gets its window decorator, then add a filter of "Crop/Mask" to remove that (increase "Top" value). Add a filter of "Chroma Key", setting the color to #0000ff and enough Similarity to remove the blue screen: I needed 270.

## Testing

Use `cargo test -- --nocapture` to see the `println!(..)`s.

## Model

Using Qwen3-VL-8B-Instruct with Q4_K_M (plus f16 mmproj) from here: https://huggingface.co/Qwen/Qwen3-VL-8B-Instruct-GGUF

## Hotkey

On Linux under Wayland, I had to make sure I was in the "input" group, by running `sudo usermod -aG input $USER` and restarting.

## Twitch

You will need to create an account for your kart_stocks bot on Twitch, if you haven't already, after which you will need to "Register Your Application" at [the Twitch developer console](https://dev.twitch.tv/console), such that you end up with a client id and secret. You will also need to set a "Redirect URI", of which the default settings of this application is pointing to "http://localhost:3000".

## Settings

You will need settings specific to your own account and setup, so copy `settings.toml.example` in the repo as simply `settings.toml` and edit it such that any empty values are given the appropriate fields.
