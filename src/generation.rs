use rand::{thread_rng, Rng};
use std::f32::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct Star {
    x: f32,
    y: f32,
    pub size: f64,
    pub star_type: u8,
    pub pos: egui::Pos2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Galaxy {
    pub stars: Vec<Star>,
}

impl Default for Galaxy {
    fn default() -> Self {
        Galaxy::new(1000, 8, 0.15)
    }
}

impl Galaxy {
    pub fn new(num_stars: usize, arms: usize, spiral_factor: f32) -> Self {
        let mut stars = Vec::with_capacity(num_stars);

        for i in 0..num_stars {
            let angle = i as f32 * spiral_factor + (i % arms) as f32 * (2.0 * PI / arms as f32);
            let radius = i as f32 * 0.01;

            let x = radius * angle.cos();
            let y = radius * angle.sin();
            let size = 1.0 + (i % 5) as f64;
            let pos = egui::Pos2::new(x * 200.0 + 400.0, y * 200.0 + 400.0);
            let star_type = thread_rng().gen_range(0..3);

            stars.push(Star {
                x,
                y,
                size,
                star_type,
                pos,
            });
        }

        Galaxy { stars }
    }
}
