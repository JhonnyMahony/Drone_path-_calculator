use std::f64::consts::PI;

use crate::models::{Obstacle, Point};
use ordered_float::OrderedFloat;
use plotters::prelude::*;

// Function to visualize the path
pub fn visualize_path(
    route: &[Point],
    obstacles: &[Obstacle],
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("drone_route.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Find the min and max x and y coordinates from the route
    let (min_x, max_x) = (0.0, 100.0);
    let (min_y, max_y) = (0.0, 50.0);
    // Build the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Drone Optimal Route", ("sans-serif", 50).into_font())
        .margin(1)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    // Configure the mesh (grid)
    chart.configure_mesh().draw()?;

    for obstacle in obstacles {
        let num_points = 20; // Number of points to approximate the circle (higher = smoother)
        let angle_step = 2.0 * PI / num_points as f64; // Step angle between points

        // Generate edge points in a circular shape around the obstacle's center
        let points: Vec<(f64, f64)> = (0..num_points)
            .map(|i| {
                let angle = i as f64 * angle_step;
                let x = obstacle.center.x.0 + obstacle.radius * angle.cos();
                let y = obstacle.center.y.0 + obstacle.radius * angle.sin();
                (x, y)
            })
            .collect();

        // Draw the obstacle as a polygon
        chart.draw_series(std::iter::once(Polygon::new(
            points,
            ShapeStyle {
                color: RED.mix(0.3),
                filled: true,
                stroke_width: 2,
            },
        )))?;
    }
    // Draw the drone path as a line
    let drone_path: Vec<(f64, f64)> = route
        .iter()
        .map(|p| (p.x.0, p.y.0)) // Unwrap OrderedFloat for plotting
        .collect();
    chart.draw_series(LineSeries::new(drone_path, &BLUE))?;

    root.present()?; // Present the image
    Ok(())
}
