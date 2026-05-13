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
    is_buy: bool,
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
    let mut values: Vec<i32> = vec![];
    let mut racing: bool = false;
    let mut next_is_new: bool = false;
    let mut actions: HashMap<uuid::Uuid, ActionText> = HashMap::new();

    let box_size = (
        settings.window.width as f32,
        settings.window.height as f32 * 0.5,
    );
    let box_top = settings.window.height as f32 - box_size.1;

    loop {
        if let Ok(val) = receive_from_run.try_recv() {
            value = val;
            if next_is_new {
                values.clear();
                next_is_new = false;
            }
            if racing {
                values.push(val);
            }
        }
        if let Ok(action) = receive_from_twitch.try_recv() {
            let action_text = ActionText {
                value: action.value,
                start_time: std::time::Instant::now(),
                x_pos: ::rand::rng().random_range(20.0..=box_size.0 - 40.0),
                special: false,
                is_buy: action.is_buy,
            };
            actions.insert(uuid::Uuid::new_v4(), action_text);
        }
        if let Ok(race_state_change) = receive_from_hotkey.try_recv() {
            match race_state_change {
                Some(val) => {
                    let action_text = ActionText {
                        value: val,
                        start_time: std::time::Instant::now(),
                        x_pos: (box_size.0 * 0.5) - 50.0,
                        special: true,
                        is_buy: false,
                    };
                    actions.insert(uuid::Uuid::new_v4(), action_text);
                    racing = false;
                }
                None => {
                    next_is_new = true;
                    racing = true;
                }
            };
        }

        clear_background(GRAY);

        border(
            (0.0, 0.0),
            (
                settings.window.width as f32,
                settings.window.height as f32 * 0.5,
            ),
            settings.window.border,
            DARKGRAY,
        );

        let graph_space = settings.window.border * 2.0;
        line_graph(
            (graph_space, graph_space),
            (
                settings.window.width as f32 - (graph_space * 2.0),
                (settings.window.height as f32 * 0.5) - (graph_space * 2.0),
            ),
            &values,
        );

        border(
            (0.0, box_top - settings.window.border),
            (
                settings.window.width as f32,
                (settings.window.height as f32 * 0.5) - (settings.window.border * 1.5),
            ),
            settings.window.border,
            DARKGRAY,
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

        animated_actions(&settings, &mut actions, box_top, &regular_font, &bold_font);

        next_frame().await
    }
}

fn border(position: (f32, f32), size: (f32, f32), thickness: f32, color: Color) {
    draw_rectangle(position.0, position.1, size.0, thickness, color); // top
    draw_rectangle(position.0, position.1, thickness, size.1, color); // left
    draw_rectangle(
        (position.0 + size.0) - thickness,
        position.1,
        thickness,
        size.1,
        color,
    ); // right
    draw_rectangle(
        position.0,
        (position.1 + size.1) - thickness,
        size.0,
        thickness,
        color,
    ); // bottom
}

fn line_graph(position: (f32, f32), size: (f32, f32), values: &[i32]) {
    if values.is_empty() {
        return;
    }
    let min_value = *values.iter().min().unwrap() as f32;
    let max_value = *values.iter().max().unwrap() as f32;
    let range = max_value - min_value;
    let mut points: Vec<(f32, f32)> = vec![];
    for i in 0..values.len() {
        let x: f32 = ((i as f32 / (values.len() - 1) as f32) * size.0) + position.0;
        let norm = if range > 0.0 {
            (values[i] as f32 - min_value) as f32 / range
        } else {
            0.5
        };
        let y: f32 = ((1.0 - norm) * size.1) + position.1;
        points.push((x, y));
    }
    for i in 0..(points.len() - 1) {
        draw_line(
            points[i].0,
            points[i].1,
            points[i + 1].0,
            points[i + 1].1,
            3.0,
            if points[i].1 > points[i + 1].1 {
                GREEN
            } else if points[i].1 < points[i + 1].1 {
                RED
            } else {
                WHITE
            },
        );
    }
    for point in &points {
        draw_circle(point.0, point.1, 4.5, WHITE);
    }
}

fn animated_actions(
    settings: &crate::settings::Settings,
    actions: &mut HashMap<uuid::Uuid, ActionText>,
    box_top: f32,
    regular_font: &Font,
    bold_font: &Font,
) {
    let mut actions_to_delete: Vec<uuid::Uuid> = vec![];
    for (id, action) in actions.iter() {
        let (prefix, font_size, color, font) = if action.is_buy {
            ("", 40, WHITE, regular_font)
        } else if action.special {
            ("", 100, YELLOW, bold_font)
        } else if action.value < 0 {
            ("-", 40, RED, regular_font)
        } else {
            ("+", 40, GREEN, regular_font)
        };
        let time = action.start_time.elapsed().as_millis();
        let end_time = settings.window.float_time;
        let time_through = time as f32 / end_time as f32;
        draw_text_ex(
            &format!("{}${}", prefix, action.value.abs()),
            action.x_pos,
            (box_top - 20.0) - (100.0 * time_through),
            TextParams {
                font_size: font_size,
                color: Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: 1.0 - time_through,
                },
                font: Some(font),
                ..Default::default()
            },
        );
        if time >= end_time {
            actions_to_delete.push(*id);
        }
    }
    for id in &actions_to_delete {
        actions.remove(id);
    }
}
