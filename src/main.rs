use nannou::{prelude::*, window::Id};
use std::collections::LinkedList;

const NUM_SEGMENTS: u8 = 5;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    arm: Arm,
    _window_id: Id,
}

fn model(app: &App) -> Model {
    // Create a new window!
    let new_window = app.new_window().size(512, 512).view(view).build().unwrap();

    let win = app.window_rect();
    let mut arm = Arm {
        segments: LinkedList::new(),
        base: pt2(0.0, 0.0),
        // base: pt2(0.0, win.bottom()),
    };

    for _ in 0..NUM_SEGMENTS {
        arm.segments.push_back(Segment {
            base: pt2(0.0, win.bottom()),
            len: win.y.end / NUM_SEGMENTS.to_f32().unwrap() * 0.9,
            angle: 0.0,
        });
    }

    arm.update_bases();

    Model {
        arm: arm,
        _window_id: new_window,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(SLATEGREY);

    let draw = app.draw();

    for segment in model.arm.segments.iter() {
        draw.line()
            .start(segment.base)
            .end(segment.calculate_head())
            .color(WHITE)
            .stroke_weight(2.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // model.arm.track(pt2(0.0,0.0));
    model.arm.track(app.mouse.position());
    model.arm.update();
}

fn offset(length: f32, angle: f32) -> Point2 {
    pt2(length * angle.cos(), length * angle.sin())
}

pub struct Arm {
    base: Point2,
    segments: LinkedList<Segment>,
}

impl Arm {
    pub fn update(&mut self) {
        self.update_bases();
    }

    fn update_bases(&mut self) {
        let mut current_base = self.base;
        for segment in self.segments.iter_mut() {
            segment.base = current_base;
            current_base = segment.calculate_head();
        }
    }

    pub fn track(&mut self, point: Point2) {
        let mut current_target = point;
        for segment in self.segments.iter_mut().rev() {
            let ray = current_target - segment.base;
            segment.angle = ray.y.atan2(ray.x);
            let change = offset(segment.len, segment.angle);
            segment.base = pt2(current_target.x - change.x, current_target.y - change.y);
            current_target = segment.base;
        }
    }
}

pub struct Segment {
    base: Point2,
    len: f32,
    angle: f32,
}

impl Segment {
    pub fn calculate_head(&self) -> Point2 {
        // let dx = self.len * self.angle.cos();
        // let dy = self.len * self.angle.sin();
        let change = offset(self.len, self.angle);
        pt2(self.base.x + change.x, self.base.y + change.y)
    }
}