mod error_handling;
mod borrow_struct;
mod shapes;
use std::{fmt::Display, str::FromStr};

use crate::shapes::{area::Area, circle::Circle, collisions::{Collidable, Contains, Points}, rectangle::Rectangle};
use anyhow::Result;

fn practice(value: Vec<usize>, index: usize) -> usize {
    return value.get(index).unwrap_or(&index) * 5;
}

enum Shape { 
    Circle(Circle),
    Rectangle(Rectangle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));
        match shape {
            "rect" => return Ok(Shape::Rectangle(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape")),
        }
    }
}

impl Points for Shape {
    fn points (&self) -> shapes::collisions::PointIter {
       match self {
        Shape::Circle(c) => c.points(),
        Shape::Rectangle(r) => return r.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rectangle(r) => return r. contains_point(point)
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rectangle(r) => return write!(f, "{}", r),
        }
    }
}

fn main() -> Result<()> {
    let arg2 = std::env::args().nth(1).expect("Filename must be passed in");
    let data: Vec<usize> = vec![1,2,3,4,5];
    // let data2: Vec<usize> = [1,2,3,4,5].to_vec();
    let response = practice(data, 0);
    let module: String = "whatever".to_string();
    println!("{}", response);

    let thing: String = error_handling::module_function(module);
    println!("{}", thing);

    error_handling::read_file(arg2);

    // borrow_struct::borrow();

    let rect = Rectangle::default();
    

    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0
    };

    println!("{}", circle.area());
    println!("{}", rect);
    println!("{}", rect.to_string());
    // println!("{}", 5.9.area());

    let rect1 = Rectangle::default();
    let rect2 = Rectangle::default();

    let circle1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };

    let circle2 = Circle {
        x: 1.5,
        y:1.5,
        radius: 2.5
    };

    rect1.collide(&rect2);
    circle1.collide(&circle2);
    rect1.collide(&circle1);

    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    let collisions = shapes
        .iter()
        .skip(1)
        .zip(shapes
            .iter()
            .take(shapes.len() -1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| {
            println!("{} collides with {}", a, b);
});

    println!("{:?}", collisions);
    
    return Ok(());

    }
