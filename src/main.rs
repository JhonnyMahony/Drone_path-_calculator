mod models;
mod visualize_path;
use models::{DronePath, Obstacle, Point};
use std::error::Error;
use std::io::{self, Write};
use visualize_path::visualize_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Введіть початкову точку
    let start = input_point("Введіть початкову точку (x y z): ")?;
    // Введіть кінцеву точку
    let end = input_point("Введіть кінцеву точку (x y z): ")?;
    // Введіть максимальну висоту
    let max_height = input_f64("Введіть максимальну висоту: ")?;
    let drone = DronePath::new(start.clone(), end.clone(), max_height);

    // Введення перешкод
    let obstacles = input_obstacles()?;
    // Введення діапазону по осі x
    let min_max_x = input_range("Введіть діапазон по осі x (мін макс): ")?;
    // Введення діапазону по осі y
    let min_max_y = input_range("Введіть діапазон по осі y (мін макс): ")?;

    // Розрахунок маршруту
    let route_option = drone.calculate_route(&obstacles);

    // Обробка Option і візуалізація шляху, якщо маршрут знайдено
    if let Some((route, _cost)) = route_option {
        visualize_path(&route, &obstacles, min_max_x, min_max_y)?;
    } else {
        println!("Маршрут не знайдено!");
    }

    Ok(())
}

// Функція для введення точки
fn input_point(prompt: &str) -> Result<Point, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?; // Вивести запит негайно

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let coords: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if coords.len() == 3 {
        Ok(Point::new(coords[0], coords[1], coords[2]))
    } else {
        Err("Некоректне введення для точки. Введіть рівно 3 значення.".into())
    }
}

// Функція для введення числа з плаваючою комою
fn input_f64(prompt: &str) -> Result<f64, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?; // Вивести запит негайно

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

// Функція для введення декількох перешкод
fn input_obstacles() -> Result<Vec<Obstacle>, Box<dyn Error>> {
    let mut obstacles = Vec::new();

    loop {
        println!("Додайте перешкоду (x y z радіус висота), або введіть 'готово' для завершення:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("готово") {
            break;
        }

        let values: Vec<f64> = trimmed
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        if values.len() == 5 {
            let point = Point::new(values[0], values[1], values[2]);
            let radius = values[3];
            let height = values[4];
            obstacles.push(Obstacle::new(point, radius, height));
        } else {
            println!("Некоректне введення для перешкоди. Введіть рівно 5 значень.");
        }
    }

    Ok(obstacles)
}

// Функція для введення діапазону
fn input_range(prompt: &str) -> Result<(f64, f64), Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?; // Вивести запит негайно

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let bounds: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    if bounds.len() == 2 {
        Ok((bounds[0], bounds[1]))
    } else {
        Err("Некоректне введення для діапазону. Введіть рівно 2 значення.".into())
    }
}
