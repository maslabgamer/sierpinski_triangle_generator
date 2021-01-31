use rand::Rng;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use image::{ImageBuffer, Rgba};

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(max_x: usize, max_y: usize) -> Self {
        let mut rng = rand::thread_rng();

        Coordinate {
            x: rng.gen_range(0..max_x),
            y: rng.gen_range(0..max_y)
        }
    }

    fn halfway_between(&self, other: &Coordinate) -> Coordinate {
        Coordinate {
            x: (self.x + other.x) / 2,
            y: (self.y + other.y) / 2
        }
    }
}

fn main() {
    let x_dimension = 1024;
    let y_dimension = 1024;
    let iterations = 10_000;

    let mut image_points: HashMap<(usize, usize), Coordinate> = HashMap::with_capacity(iterations);

    // Pick three random points on our image
    let triangle_points = vec![
        Coordinate::new(x_dimension, y_dimension),
        Coordinate::new(x_dimension, y_dimension),
        Coordinate::new(x_dimension, y_dimension),
    ];

    // Now pick our starting trace point
    let mut trace_point = Coordinate::new(x_dimension, y_dimension);
    image_points.insert((trace_point.x, trace_point.y), trace_point.clone());

    for _ in 0..iterations {
        let move_towards_point = triangle_points.choose(&mut rand::thread_rng()).unwrap();
        trace_point = trace_point.halfway_between(move_towards_point);
        image_points.insert((trace_point.x, trace_point.y), trace_point.clone());
    }

    let mut image_generation: Vec<u8> = Vec::with_capacity(x_dimension * y_dimension * 4);

    for x in 0..x_dimension {
        for y in 0..y_dimension {
            image_generation.extend_from_slice(match image_points.get(&(x, y)) {
                None => &[0, 0, 0, 255],
                Some(_) => &[255; 4]
            });
        }
    }

    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
        x_dimension as u32,
        y_dimension as u32,
        &image_generation[..]
    ).unwrap();
    image.save("image.png").unwrap();
}
