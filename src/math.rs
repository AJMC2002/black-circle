use rand::{seq::SliceRandom, thread_rng};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn is_inside(&self, circle: &Circle) -> bool {
        self.distance_to(&circle.center) <= circle.radius
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    fn trivial(boundary: &Vec<Point>) -> Self {
        match boundary.len() {
            0 => Circle {
                center: Point { x: 0.0, y: 0.0 },
                radius: 0.0,
            },
            1 => Self::from1(&boundary[0]),
            2 => Self::from2(&boundary[0], &boundary[1]),
            3 => Self::from3(&boundary[0], &boundary[1], &boundary[2]),
            _ => panic!("R should contain at most 3 points."),
        }
    }

    fn from1(p1: &Point) -> Self {
        Circle {
            center: p1.clone(),
            radius: 0.0,
        }
    }

    fn from2(p1: &Point, p2: &Point) -> Self {
        let cx = (p1.x + p2.x) / 2.0;
        let cy = (p1.y + p2.y) / 2.0;
        let center = Point { x: cx, y: cy };
        let radius = p1.distance_to(p2) / 2.0;
        Circle { center, radius }
    }

    fn from3(p1: &Point, p2: &Point, p3: &Point) -> Self {
        let m1 = (p2.y - p1.y) / (p2.x - p1.x);
        let m2 = (p3.y - p2.y) / (p3.x - p2.x);
        let cx =
            (m1 * m2 * (p1.y - p3.y) + m2 * (p1.x + p2.x) - m1 * (p2.x + p3.x)) / (2.0 * (m2 - m1));
        let cy = (-1.0 / m1) * (cx - (p1.x + p2.x) / 2.0) + (p1.y + p2.y) / 2.0;
        let center = Point { x: cx, y: cy };
        let radius = center.distance_to(p1);
        return Circle { center, radius };
    }
}

fn welzl(points: &[Point], boundary: &Vec<Point>) -> Circle {
    if points.is_empty() || boundary.len() == 3 {
        return Circle::trivial(&boundary);
    }
    let mut rng = thread_rng();
    let mut shuffled_points = points.to_vec();
    shuffled_points.shuffle(&mut rng);
    let p = shuffled_points[0].clone();
    let c = welzl(&shuffled_points[1..], boundary);
    if p.is_inside(&c) {
        return c;
    }
    let mut new_boundary = boundary.clone();
    new_boundary.push(p.clone());
    return welzl(&shuffled_points[1..].to_vec(), &new_boundary);
}

pub fn minimal_circle(points: &[Point]) -> Circle {
    welzl(&points, &mut vec![])
}
