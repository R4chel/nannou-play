use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    let xs = 0..50;
    let ys = 0..50;

    let multiplier: f32 = 10.0;
    let points = xs
        .map(|i| {
            let x = i as f32;
            ys.clone().map(move |j| {
                let y = j as f32;
                let point = pt2(x * multiplier, y * multiplier);
                (point, STEELBLUE)
            })
        })
        .flatten();

    draw.polyline().weight(3.0).points_colored(points);
    let points = (0..500).map(|i| {
        let x = i as f32 / 10.0 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, (x).cos()) * 20.0; //scale sine wave by 20.0
        (point, ORANGE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
