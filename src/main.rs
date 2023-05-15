use egui::Slider;
use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let conf = window_conf();
    let mut state = State { time: 0. };
    let mut then = std::time::SystemTime::now();

    let mut amp = 4.;
    let mut freq = 100.;
    let mut speed = 100.;

    // Decrease to improve performance on low-end systems
    let point_count = 10000;

    let gap_width: f32 = conf.window_width as f32 / point_count as f32;
    let mut points: Vec<Vec2> = vec![];

    for i in 0..point_count {
        points.push(vec2(i as f32 * gap_width, (conf.window_height / 2) as f32));
    }

    let mut show_ui = true;
    loop {
        clear_background(BLACK);

        state.time += then.elapsed().unwrap().as_secs_f32() * speed;

        // Handle Input
        if is_key_pressed(KeyCode::H) { show_ui = !show_ui }

        // Handle UI
        if show_ui {
            egui_macroquad::ui(|egui_ctx| {
                egui::Window::new("Settings")
                    .resizable(false)
                    .show(egui_ctx, |ui| {
                        ui.add(
                            Slider::new(&mut amp, 1_f32..=250_f32)
                                .text("Amplitude")
                        );
                        ui.add(
                            Slider::new(&mut freq, 1_f32..=1000_f32)
                                .text("Frequency")
                                .drag_value_speed(0.01)
                        );
                        ui.add(
                            Slider::new(&mut speed, 1_f32..=1000_f32)
                                .text("Speed")
                        );
                    });
            });
        }

        // Handle points
        for i in 0..points.len() {
            let mut point = &mut points[i];
            point.y = (((i as f32 / (point_count - 1) as f32 * freq) + state.time).sin() * amp)
                + (conf.window_height / 2) as f32;
        }

        for i in 0..points.len() {
            if i == points.len() - 1 { break; }

            let point = &points[i];
            let next_point = &points[i + 1];
            draw_line(
                point.x,
                point.y,
                next_point.x,
                next_point.y,
                2.,
                color_u8!(255, 255, 255,
                    100 + (((point.y - (conf.window_height / 2) as f32).abs() / amp) * 155.) as u8)
            );
        }

        // Draw UI
        if show_ui { egui_macroquad::draw() }

        then = std::time::SystemTime::now();
        next_frame().await;
    }
}

struct State {
    time: f32,
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Sine Wave"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}
