use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    const CLOSEST_POINTS_LIMIT: usize = 10;

    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z,
        }
    }

    fn closest_points(&self, points: Vec<Point>) -> Vec<Point> {
        // Using a binary heap to keep track of the closest points instead of Vec for performance purposes even if it uses more memory.
        let mut heap = BinaryHeap::with_capacity(Self::CLOSEST_POINTS_LIMIT + 1);

        for &point in &points {
            let distance = self.distance_to(point);
            if heap.len() < Self::CLOSEST_POINTS_LIMIT {
                heap.push(Reverse(DistancePoint { distance, point }));
            } else if let Some(mut top) = heap.peek_mut() {
                if distance < top.0.distance {
                    top.0.distance = distance;
                    top.0.point = point;
                }
            }
        }

        heap.into_iter().map(|Reverse(dp)| dp.point).collect()
    }

    fn distance_to(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    fn random() -> Point {
        Point::new(
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(0.0..1.0),
        )
    }
}

#[derive(Debug, PartialEq)]
struct DistancePoint {
    distance: f64,
    point: Point,
}

impl Eq for DistancePoint {}

impl Ord for DistancePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap().reverse()
    }
}

impl PartialOrd for DistancePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
