use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

/* MODEL */
struct Model {
    debug: bool,
    charges: Vec<Charge>,
    scalingFactor: f32,
}

fn model(_app: &App) -> Model {
    Model {
        debug: false,
        scalingFactor: 10.,
        charges: vec![],
    }
}

struct Charge {
    p: Point2,
    charge: f32,
}

impl Charge {
    fn strengthAtPoint(&self, p: Point2) -> Vec2 {
        let distance = self.p.distance(p);
        let strength = self.charge / (distance.pow(2));
        pt2(
            strength * (p.x - self.p.x) / distance,
            strength * (p.y - self.p.y) / distance,
        )
    }
}
struct Direction {
    v: Vec2,
}

impl Direction {
    fn apply(&self, p: Point2) -> Point2 {
        todo!()
    }

    fn renderable(&self) -> (Point2, Point2) {
        todo!()
    }
}

/* UPDATE */

fn directionAtPoint(model: &Model, p: Point2) -> Direction {
    let v: Vec<Vec2> = model
        .charges
        .iter()
        .map(|charge| charge.strengthAtPoint(p))
        .collect();
    Direction { v: v.iter().sum() }
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

    draw.to_frame(app, &frame).unwrap();
}
