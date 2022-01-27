use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

/* MODEL */
struct Model {
    debug: bool,
}

fn model(_app: &App) -> Model {
    Model { debug: false }
}

struct Direction {}

impl Direction {
    fn apply(&self, p: Point2) -> Point2 {
        todo!()
    }

    fn renderable(&self) -> (Point2, Point2) {
        todo!()
    }
}

/* UPDATE */

fn directionAtPoint(p: Point2) -> Direction {
    todo!()
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

/* VIEW */

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // let area = app.window_rect();
    // Draw field
    if model.debug {}
    let xs = -50..50;
    let ys = -50..50;

    let multiplier: f32 = 20.0;
    let points: Vec<Point2> = xs
        .map(|i| {
            let x = i as f32;
            ys.clone().map(move |j| {
                let y = j as f32;
                let point = pt2(x * multiplier, y * multiplier);
                point
            })
        })
        .flatten()
        .collect();

    points.iter().for_each(|p| {
        let end = pt2(p.x + 10.0, p.y + 10.0);
        draw.arrow().points(*p, end);
    });
    // draw.polyline().weight(3.0).points_colored(points);
    // let points = (0..500).map(|i| {
    //     let x = i as f32 / 10.0 - 25.0; //subtract 25 to center the sine wave
    //     let point = pt2(x, (x).cos()) * 20.0; //scale sine wave by 20.0
    //     (point, ORANGE)
    // });
    // draw.polyline().weight(3.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
