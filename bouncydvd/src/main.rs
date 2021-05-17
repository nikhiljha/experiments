use nannou::prelude::*;
use nannou::rand;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
    y: f32,
    angle: f32,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("dvd.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        x: 0.0,
        y: 0.0,
        angle: 90.0.to_radians(),
        texture: texture,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {    
    model.x += model.angle.cos() * 0.01;
    model.y += model.angle.sin() * 0.01;
    if model.x >= 1.0 {
       model.angle = rand::random_range(90.0.to_radians(), 270.0.to_radians());
    } else if model.y >= 1.0 {
       model.angle = rand::random_range(180.0.to_radians(), 360.0.to_radians());
    } else if model.x <= -1.0 {
       model.angle = rand::random_range(-90.0.to_radians(), 90.0.to_radians());
    } else if model.y <= -1.0 {
       model.angle = rand::random_range(0.0.to_radians(), 180.0.to_radians());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect().pad_top(31.0).pad_bottom(31.0).pad_left(50.0).pad_right(50.0);

    frame.clear(WHITE);
    // draw.text(&format!("{} {} {}", model.angle, model.x, model.y))
    //     .x_y(0.0, 0.0)
    //     .color(BLACK);
    // draw.rect()
    //     .x_y(model.x * win.w() / 2.0, model.y * win.h() / 2.0)
    //     .w_h(100.0, 100.0)
    //     .color(PLUM);
    
    draw.texture(&model.texture)
        .x_y(model.x * win.w() / 2.0, model.y * win.h() / 2.0)
        .w_h(100.0, 62.0);

    draw.to_frame(app, &frame).unwrap();
}