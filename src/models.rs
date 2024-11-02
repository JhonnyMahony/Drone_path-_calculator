use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

// Derive Eq and Hash for Point using OrderedFloat instead of f64
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub z: OrderedFloat<f64>, // Height
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            z: OrderedFloat(z),
        }
    }
}

pub struct DronePath {
    pub start: Point,
    pub end: Point,
    pub max_height: OrderedFloat<f64>,
}

impl DronePath {
    pub fn new(start: Point, end: Point, max_height: f64) -> Self {
        Self {
            start,
            end,
            max_height: OrderedFloat(max_height),
        }
    }

    pub fn calculate_route(
        &self,
        obstacles: &[Obstacle],
    ) -> Option<(Vec<Point>, OrderedFloat<f64>)> {
        let start = self.start.clone();
        let end = self.end.clone();

        // A* search function for neighbors
        let result = astar(
            &start,
            |p| self.successors(p, obstacles),
            |p| OrderedFloat(self.heuristic(p, &end)),
            |p| *p == end,
        );

        result
    }

    // Generate neighbors (possible moves)
    fn successors(
        &self,
        current: &Point,
        obstacles: &[Obstacle],
    ) -> Vec<(Point, OrderedFloat<f64>)> {
        let mut successors = Vec::new();

        let step_size = 0.5; // Reduced step size for more accurate pathfinding

        // Try moving in the x, y, and z directions (including diagonals)
        let directions = vec![
            (step_size, 0.0, 0.0),
            (-step_size, 0.0, 0.0),
            (0.0, step_size, 0.0),
            (0.0, -step_size, 0.0),
            (step_size, step_size, 0.0),
            (step_size, -step_size, 0.0),
            (-step_size, step_size, 0.0),
            (-step_size, -step_size, 0.0),
            (step_size, 0.0, step_size),
            (step_size, 0.0, -step_size),
            (-step_size, 0.0, step_size),
            (-step_size, 0.0, -step_size),
            (0.0, step_size, step_size),
            (0.0, step_size, -step_size),
            (0.0, -step_size, step_size),
            (0.0, -step_size, -step_size),
        ];

        for (dx, dy, dz) in directions {
            let next = Point::new(current.x.0 + dx, current.y.0 + dy, current.z.0 + dz);

            // Check if this move collides with any obstacle
            if !self.collides_with_obstacles(&next, obstacles) {
                let cost = OrderedFloat(self.distance(current, &next));
                successors.push((next, cost));
            }
        }

        successors
    }

    // Heuristic function for A* (Euclidean distance to goal)
    fn heuristic(&self, current: &Point, goal: &Point) -> f64 {
        self.distance(current, goal)
    }

    // Check if the point collides with any obstacle
    fn collides_with_obstacles(&self, point: &Point, obstacles: &[Obstacle]) -> bool {
        for obstacle in obstacles {
            let dist = self.distance_2d(point, &obstacle.center);

            // Check if point is within obstacle's radius and under its height
            if dist < obstacle.radius && point.z.0 <= obstacle.height {
                return true;
            }
        }
        false
    }

    // Euclidean distance in 3D
    fn distance(&self, a: &Point, b: &Point) -> f64 {
        ((a.x.0 - b.x.0).powi(2) + (a.y.0 - b.y.0).powi(2) + (a.z.0 - b.z.0).powi(2)).sqrt()
    }

    // Euclidean distance in 2D (ignores height)
    fn distance_2d(&self, a: &Point, b: &Point) -> f64 {
        ((a.x.0 - b.x.0).powi(2) + (a.y.0 - b.y.0).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub center: Point,
    pub radius: f64,
    pub height: f64,
}

impl Obstacle {
    pub fn new(center: Point, radius: f64, height: f64) -> Self {
        Self {
            center,
            radius,
            height,
        }
    }
}
