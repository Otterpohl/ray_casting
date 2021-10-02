use ::rand::Rng;
use macroquad::prelude::*;

extern crate rand;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ray Casting".to_owned(),
        window_width: 1800,
        window_height: 1200,
        ..Default::default()
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(f32, f32);

// struct for Walls/obstacles
struct Boundary {
    a: Point,
    b: Point,
}

impl Boundary {
    fn new(a: Point, b: Point) -> Self {
        Boundary { a: a, b: b }
    }

    fn show(&self) {
        draw_line(self.a.0, self.a.1, self.b.0, self.b.1, 1.0, WHITE);
    }
}

// Struct for individual rays
#[derive(Debug, Copy, Clone)]
struct Ray {
    position: Point,
    direction: Point,
}

impl Ray {
    fn new(position: Point, direction: Point) -> Self {
        Ray {
            position: position,
            direction: direction,
        }
    }

    fn cast(&self, wall: &Boundary) -> Option<Point> {
        let denominator =
            (wall.a.0 - wall.b.0) * self.direction.1 - (wall.a.1 - wall.b.1) * self.direction.0;

        if denominator == 0.0 {
            return None;
        }

        let t = ((wall.a.0 - self.position.0) * self.direction.1
            - (wall.a.1 - self.position.1) * self.direction.0)
            / denominator;

        let u = -((wall.a.0 - wall.b.0) * (wall.a.1 - self.position.1)
            - (wall.a.1 - wall.b.1) * (wall.a.0 - self.position.0))
            / denominator;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            return Some(Point(
                wall.a.0 + t * (wall.b.0 - wall.a.0),
                wall.a.1 + t * (wall.b.1 - wall.a.1),
            ));
        } else {
            return None;
        }
    }
}

// struct for "light" source
#[derive(Debug, Clone)]
struct Particle {
    position: Point,
    rays: Vec<Ray>,
}

impl Particle {
    fn new() -> Self {
        Particle {
            position: Point(mouse_position().0, mouse_position().1),
            rays: vec![],
        }
    }

    fn set_rays(&mut self) {
        let mut i = 0.0;

        while i < 360.0 {
            self.rays.push(Ray::new(
                Point(mouse_position().0, mouse_position().1),
                Point(
                    f32::cos((i as f32).to_radians()),
                    f32::sin((i as f32).to_radians()),
                ),
            ));

            i += 0.1;
        }
    }

    fn cast(&self, walls: &Vec<Boundary>) {
        let mut closest: Option<Point> = None;
        for ray in self.rays.iter() {
            let mut record = f32::INFINITY;
            for wall in walls.iter() {
                let point = ray.cast(wall);
                if point.is_some() {
                    let distance = distance(&point.unwrap(), &self.position);
                    if distance < record {
                        record = distance;
                        closest = point;
                    }
                }
            }

            if closest.is_some() {
                draw_line(
                    self.position.0,
                    self.position.1,
                    closest.unwrap().0,
                    closest.unwrap().1,
                    7.0,
                    Color::new(1.00, 1.00, 1.00, 0.02),
                );
            }
        }
    }

    fn update(&mut self, walls: &Vec<Boundary>) {
        self.set_rays();
        self.cast(walls);

        for wall in walls.iter() {
            wall.show();
        }
    }
}

fn distance(a: &Point, b: &Point) -> f32 {
    (f32::powf(a.0 - b.0, 2.0) + f32::powf(a.1 - b.1, 2.0)).sqrt()
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut walls = vec![];

    loop {
        clear_background(BLACK);
        // if space is pressed then generate a new set of walls
        if is_key_pressed(macroquad::input::KeyCode::Space) {
            // clear the previous vec of walls
            walls.clear();

            // draw the borders of the scene
            walls.push(Boundary::new(Point(0.0, 0.0), Point(0.0, screen_height())));
            walls.push(Boundary::new(Point(0.0, 0.0), Point(screen_width(), 0.0)));
            walls.push(Boundary::new(
                Point(0.0, screen_height()),
                Point(screen_width(), screen_height()),
            ));
            walls.push(Boundary::new(
                Point(screen_width(), 0.0),
                Point(screen_width(), screen_height()),
            ));

            // draw random walls
            for _ in 0..5 {
                let mut rng = rand::thread_rng();
                let x1 = rng.gen_range(0.0..=screen_width());
                let y1 = rng.gen_range(0.0..=screen_height());
                let x2 = rng.gen_range(0.0..=screen_width());
                let y2 = rng.gen_range(0.0..=screen_height());

                walls.push(Boundary::new(Point(x1, y1), Point(x2, y2)));
            }
        }

        // draw a light particle at the mouse location
        Particle::new().update(&walls);

        next_frame().await;
    }
}
