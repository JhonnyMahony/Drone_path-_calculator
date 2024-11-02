use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

// Додано реалізацію Eq та Hash для Point, використовуючи OrderedFloat замість f64
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub z: OrderedFloat<f64>, // Висота
}

impl Point {
    // Створює нову точку з заданими координатами x, y, z
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
    // Створює новий шлях дрона із заданою початковою та кінцевою точками, а також максимальною висотою
    pub fn new(start: Point, end: Point, max_height: f64) -> Self {
        Self {
            start,
            end,
            max_height: OrderedFloat(max_height),
        }
    }

    // Обчислює маршрут з урахуванням перешкод
    pub fn calculate_route(
        &self,
        obstacles: &[Obstacle],
    ) -> Option<(Vec<Point>, OrderedFloat<f64>)> {
        let start = self.start.clone();
        let end = self.end.clone();

        // Функція пошуку A* для знаходження сусідів
        let result = astar(
            &start,
            |p| self.successors(p, obstacles),
            |p| OrderedFloat(self.heuristic(p, &end)),
            |p| *p == end,
        );

        result
    }

    // Генерує сусідні точки (можливі переміщення)
    fn successors(
        &self,
        current: &Point,
        obstacles: &[Obstacle],
    ) -> Vec<(Point, OrderedFloat<f64>)> {
        let mut successors = Vec::new();

        let step_size = 0.5; // Зменшений крок для більш точного пошуку шляху

        // Спробуємо рухатися в напрямках x, y та z (включаючи діагоналі)
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

            // Перевірка, чи не перетинається це переміщення з перешкодами
            if !self.collides_with_obstacles(&next, obstacles) {
                let cost = OrderedFloat(self.distance(current, &next));
                successors.push((next, cost));
            }
        }

        successors
    }

    // Евристична функція для A* (евклідова відстань до мети)
    fn heuristic(&self, current: &Point, goal: &Point) -> f64 {
        self.distance(current, goal)
    }

    // Перевіряє, чи перетинається точка з перешкодами
    fn collides_with_obstacles(&self, point: &Point, obstacles: &[Obstacle]) -> bool {
        for obstacle in obstacles {
            let dist = self.distance_2d(point, &obstacle.center);

            // Перевірка, чи знаходиться точка в радіусі перешкоди та під її висотою
            if dist < obstacle.radius && point.z.0 <= obstacle.height {
                return true;
            }
        }
        false
    }

    // Евклідова відстань в 3D
    fn distance(&self, a: &Point, b: &Point) -> f64 {
        ((a.x.0 - b.x.0).powi(2) + (a.y.0 - b.y.0).powi(2) + (a.z.0 - b.z.0).powi(2)).sqrt()
    }

    // Евклідова відстань в 2D (ігнорує висоту)
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
    // Створює нову перешкоду з заданим центром, радіусом та висотою
    pub fn new(center: Point, radius: f64, height: f64) -> Self {
        Self {
            center,
            radius,
            height,
        }
    }
}
