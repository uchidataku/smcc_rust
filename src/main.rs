use std::ptr::null;
use nannou::prelude::*;

static RADIUS: f32 = 25.0;

fn main() {
    nannou::app(model)
        .size(100, 100)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);
    let boundary = app.window_rect();
    for y in -25..=25 {
        for x in -25..=25 {
            let dx = x as f32 - 0.0;
            let dy = y as f32 - 0.0;
            let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

            match dist {
                24.9..=25.1 => draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).color(BLACK),
                24.0..=24.9 => draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).color(GRAY),
                25.1..=25.5 => draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).color(GRAY),
                _ => draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).color(WHITE),
            };
        }
    }
    draw.to_frame(app, &frame).unwrap()
}