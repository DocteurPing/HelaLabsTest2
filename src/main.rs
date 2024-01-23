use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    const CLOSEST_POINTS_LIMIT: usize = 10;

    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x: Point::clamp(x),
            y: Point::clamp(y),
            z: Point::clamp(z),
        }
    }

    fn closest_points(&self, points: Vec<Point>) -> Vec<Point> {
        let mut closest_points = points;
        let limit = closest_points.len().min(Self::CLOSEST_POINTS_LIMIT);

        closest_points.select_nth_unstable_by(limit - 1, |a, b| {
            self.distance_to(*a).partial_cmp(&self.distance_to(*b)).unwrap_or(Ordering::Equal)
        });

        closest_points.truncate(limit);
        closest_points
    }

    fn distance_to(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    fn random() -> Point {
        Point::new(rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0), rand::thread_rng().gen_range(0.0..1.0))
    }

    fn clamp(coordinate: f64) -> f64 {
        coordinate.max(0.0).min(1.0)
    }
}

fn main() {
    let points: Vec<Point> = (0..10_000_000).map(|_| Point::random()).collect();
    let test_point = Point::random();

    let start_time = std::time::Instant::now();
    let closest_points = test_point.closest_points(points);
    let elapsed_time = start_time.elapsed().as_secs_f64();

    closest_points.iter().for_each(|point| println!("{:?}", point));
    println!("Execution time: {:.6} seconds", elapsed_time);
}
