mod models;
mod visualize_path;

use models::{DronePath, Obstacle, Point};
use visualize_path::visualize_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Point::new(0.0, 0.0, 0.0);
    let end = Point::new(50.0, 50.0, 10.0);
    let max_height = 15.0;
    let drone = DronePath::new(start.clone(), end.clone(), max_height);

    let obstacles = vec![
        Obstacle::new(Point::new(20.0, 20.0, 0.0), 5.0, 15.0),
        Obstacle::new(Point::new(35.0, 35.0, 0.0), 1.0, 50.0),
        Obstacle::new(Point::new(45.0, 45.0, 20.0), 8.0, 2.0),
        Obstacle::new(Point::new(35.0, 45.0, 20.0), 4.0, 50.0),
    ];

    // Calculate the route
    let route_option = drone.calculate_route(&obstacles);

    // Handle the Option and visualize the path if a route is found
    if let Some((route, _cost)) = route_option {
        visualize_path(&route, &obstacles)?;
    } else {
        println!("No valid route found!");
    }

    Ok(())
}
