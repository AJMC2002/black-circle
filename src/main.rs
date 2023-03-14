mod math;
use math::*;
use nannou::prelude::*;
struct Model {
    circle: Circle,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIN_SIZE, WIN_SIZE)
        .view(view)
        .build()
        .unwrap();
    let circle = minimal_circle(&POINTS);

    println!("Center: ({}, {})", circle.center.x, circle.center.y);
    println!("Radius: {}", circle.radius);
    println!("Area: {}", std::f64::consts::PI * circle.radius.powi(2));

    Model { circle }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(STEELBLUE);

    draw.ellipse()
        .x_y(model.circle.center.x as f32, model.circle.center.y as f32)
        .radius(model.circle.radius as f32)
        .stroke_weight(2.0)
        .stroke_color(BLACK)
        .no_fill();

    for point in POINTS.iter() {
        draw.ellipse()
            .x_y(point.x as f32, point.y as f32)
            .radius(3.0)
            .color(PLUM)
            .stroke_weight(0.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).run();
}

const WIN_SIZE: u32 = 600;

const POINTS: [Point; 8] = [
    Point { x: -20.0, y: 40.0 },
    Point { x: 100.0, y: 35.0 },
    Point { x: 23.5, y: 34.5 },
    Point { x: 74.5, y: -35.5 },
    Point { x: -20.0, y: -30.0 },
    Point { x: 90.0, y: 72.0 },
    Point { x: -200.0, y: 35.0 },
    Point { x: 155.0, y: 80.0 },
];
