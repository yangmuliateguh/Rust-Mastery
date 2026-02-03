struct Point {
    x: i32,
    y: i32,
}

fn quadrant(p: Point) -> String {
    match p {
        Point {x, y} if x > 0 && y > 0 => "Q1".into(),
        Point {x, y} if x < 0 && y > 0 => "Q2".into(),
        Point {x, y} if x < 0 && y < 0 => "Q3".into(),
        Point {x, y} if x > 0 && y < 0 => "Q4".into(),
        _ => "Origin or Axis".into()
    }
}

fn main() {
    let p = Point {x: 3, y: -2};
    println!("{}", quadrant(p));
}