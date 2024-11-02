use crate::models::{Obstacle, Point};
use plotters::prelude::*;
use std::f64::consts::PI;

// Функція для візуалізації маршруту
pub fn visualize_path(
    route: &[Point],        // Маршрут дрона, який складається з точок
    obstacles: &[Obstacle], // Перешкоди на маршруті
    min_max_x: (f64, f64),  // Мінімальне та максимальне значення координати X
    min_max_y: (f64, f64),  // Мінімальне та максимальне значення координати Y
) -> Result<(), Box<dyn std::error::Error>> {
    // Створюємо область малювання та встановлюємо фон на білий
    let root = BitMapBackend::new("drone_route.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Витягуємо мінімальні та максимальні значення координат X та Y
    let (min_x, max_x) = min_max_x;
    let (min_y, max_y) = min_max_y;

    // Створюємо графік з параметрами
    let mut chart = ChartBuilder::on(&root)
        .caption("Оптимальний маршрут дрона", ("sans-serif", 50).into_font())
        .margin(1)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    // Налаштовуємо сітку (графічну сітку на графіку)
    chart.configure_mesh().draw()?;

    // Відображення перешкод як кіл на графіку
    for obstacle in obstacles {
        let num_points = 20; // Кількість точок для наближення кола (більше точок = більш гладке коло)
        let angle_step = 2.0 * PI / num_points as f64; // Крок кута між точками

        // Генеруємо точки на краях кола навколо центру перешкоди
        let points: Vec<(f64, f64)> = (0..num_points)
            .map(|i| {
                let angle = i as f64 * angle_step;
                let x = obstacle.center.x.0 + obstacle.radius * angle.cos();
                let y = obstacle.center.y.0 + obstacle.radius * angle.sin();
                (x, y)
            })
            .collect();

        // Малюємо перешкоду як полігон
        chart.draw_series(std::iter::once(Polygon::new(
            points,
            ShapeStyle {
                color: RED.mix(0.3),
                filled: true,
                stroke_width: 2,
            },
        )))?;
    }

    // Відображення маршруту дрона як лінії
    let drone_path: Vec<(f64, f64)> = route
        .iter()
        .map(|p| (p.x.0, p.y.0)) // Розпаковуємо OrderedFloat для відображення
        .collect();
    chart.draw_series(LineSeries::new(drone_path, &BLUE))?;

    root.present()?; // Виводимо зображення
    Ok(())
}
