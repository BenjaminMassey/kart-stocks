use ::rand::RngExt;
use macroquad::prelude::*;
use std::collections::HashMap;

pub fn start(
    settings: &crate::settings::Settings,
    receive_from_run: tokio::sync::mpsc::UnboundedReceiver<i32>,
    receive_from_twitch: tokio::sync::mpsc::UnboundedReceiver<crate::twitch::InvestmentAction>,
    receive_from_hotkey: tokio::sync::mpsc::UnboundedReceiver<Option<i32>>,
) {
    let conf = Conf {
        window_title: settings.window.title.clone(),
        window_width: settings.window.width,
        window_height: settings.window.height,
        window_resizable: false,
        ..Default::default()
    };

    let settings_for_run = settings.clone();
    let _ = std::thread::spawn(move || {
        macroquad::Window::from_config(
            conf,
            run(
                settings_for_run,
                receive_from_run,
                receive_from_twitch,
                receive_from_hotkey,
            ),
        );
    });
}

struct ActionText {
    value: i32,
    start_time: std::time::Instant,
    x_pos: f32,
    special: bool,
}

async fn run(
    settings: crate::settings::Settings,
    mut receive_from_run: tokio::sync::mpsc::UnboundedReceiver<i32>,
    mut receive_from_twitch: tokio::sync::mpsc::UnboundedReceiver<crate::twitch::InvestmentAction>,
    mut receive_from_hotkey: tokio::sync::mpsc::UnboundedReceiver<Option<i32>>,
) {
    let regular_font = load_ttf_font("./data/fonts/Roboto-Regular.ttf")
        .await
        .unwrap();
    let bold_font = load_ttf_font("./data/fonts/Roboto-Bold.ttf").await.unwrap();
    let mut value: i32 = settings.game.initial_price;
    let mut buys: HashMap<uuid::Uuid, ActionText> = HashMap::new();
    let mut sells: HashMap<uuid::Uuid, ActionText> = HashMap::new();

    let box_size = (settings.window.width as f32, 250.0);
    let box_top = settings.window.height as f32 - box_size.1;

    loop {
        if let Ok(val) = receive_from_run.try_recv() {
            value = val;
        }
        if let Ok(action) = receive_from_twitch.try_recv() {
            let action_text = ActionText {
                value: action.value,
                start_time: std::time::Instant::now(),
                x_pos: ::rand::rng().random_range(20.0..=box_size.0 - 40.0),
                special: false,
            };
            if action.is_buy {
                buys.insert(uuid::Uuid::new_v4(), action_text);
            } else {
                sells.insert(uuid::Uuid::new_v4(), action_text);
            }
        }
        if let Ok(race_state_change) = receive_from_hotkey.try_recv() {
            match race_state_change {
                Some(val) => {
                    let action_text = ActionText {
                        value: val,
                        start_time: std::time::Instant::now(),
                        x_pos: (box_size.0 * 0.5) - 50.0,
                        special: true,
                    };
                    sells.insert(uuid::Uuid::new_v4(), action_text);
                }
                None => {} // TODO: race started: behavior TBD
            };
        }
        clear_background(Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        });

        draw_rectangle(0.0, box_top, box_size.0, box_size.1, DARKGRAY);
        draw_rectangle(
            settings.window.border,
            box_top + settings.window.border,
            box_size.0 - (settings.window.border * 2.0),
            box_size.1 - (settings.window.border * 4.5),
            GRAY,
        );

        draw_text_ex(
            &format!("Cost: ${}", value),
            40.0,
            settings.window.height as f32 - 100.0,
            TextParams {
                font_size: 120,
                color: WHITE,
                font: Some(&regular_font),
                ..Default::default()
            },
        );

        let mut buys_to_delete: Vec<uuid::Uuid> = vec![];
        for (id, buy) in &buys {
            let time = buy.start_time.elapsed().as_millis();
            let end_time = settings.window.float_time;
            let time_through = time as f32 / end_time as f32;
            draw_text_ex(
                &format!("${}", buy.value),
                buy.x_pos,
                (box_top - 20.0) - (100.0 * time_through),
                TextParams {
                    font_size: 40,
                    color: Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0 - time_through,
                    },
                    font: Some(&regular_font),
                    ..Default::default()
                },
            );
            if time >= end_time {
                buys_to_delete.push(*id);
            }
        }
        for id in &buys_to_delete {
            buys.remove(id);
        }

        let mut sells_to_delete: Vec<uuid::Uuid> = vec![];
        for (id, sell) in &sells {
            let time = sell.start_time.elapsed().as_millis();
            let end_time = settings.window.float_time;
            let time_through = time as f32 / end_time as f32;
            let color = if sell.special {
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                }
            } else if sell.value < 0 {
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            } else {
                Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                }
            };
            draw_text_ex(
                &format!(
                    "{}${}",
                    if sell.special {
                        ""
                    } else if sell.value < 0 {
                        "-"
                    } else {
                        "+"
                    },
                    sell.value.abs()
                ),
                sell.x_pos,
                (box_top - 20.0) - (100.0 * time_through),
                TextParams {
                    font_size: if sell.special { 100 } else { 40 },
                    color: Color {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                        a: 1.0 - time_through,
                    },
                    font: if sell.special {
                        Some(&bold_font)
                    } else {
                        Some(&regular_font)
                    },
                    ..Default::default()
                },
            );
            if time >= end_time {
                sells_to_delete.push(*id);
            }
        }
        for id in &sells_to_delete {
            sells.remove(id);
        }

        next_frame().await
    }
}
