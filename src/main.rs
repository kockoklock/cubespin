use std::{f32::consts::PI, io::Write};

const SIZE: f32 = 40f32;

struct Point {
    x: f32,
    y: f32,
    z: f32,
    d: char
}

struct DisplayPoint {
    z: f32,
    d: char
}

enum Side {
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right
}

fn create_side(point: &Point) -> Vec<Point>
{
    let dens: u32 = SIZE as u32;
    let mut v: Vec<Point> = Vec::new();
    for x in (0..dens).map(|v| v as f32) {
        for y in (0..dens).map(|v| v as f32) {
            let fd = dens as f32;
            v.push(Point {
                x: point.x + x * SIZE / fd,
                y: point.y + y * SIZE / fd,
                z: point.z,
                d: point.d
            })
        }
    }
    v
}

fn create_all_sides(v: &mut Vec<Point>)
{
    let side_chars = ['#','$','@','"',';','.'];
    for (i, s) in [
        Side::Front,
        Side::Back,
        Side::Left,
        Side::Right,
        Side::Top,
        Side::Bottom
    ].iter().enumerate() {
        append_side(v, s, side_chars[i]);
    }
}

fn rotate_x(point: &Point, rad: f32) -> Point 
{
    Point {
        x: point.x,
        y: point.z * rad.sin() + point.y * rad.cos(),
        z: point.z * rad.cos() - point.y * rad.sin(),
        d: point.d
    }
}

fn rotate_y(point: &Point, rad: f32) -> Point 
{
    Point {
        x: point.x * rad.cos() - point.z * rad.sin(),
        y: point.y,
        z: point.x * rad.sin() + point.z * rad.cos(),
        d: point.d
    }
}

fn rotate_z(point: &Point, rad: f32) -> Point 
{
    Point {
        x: point.x * rad.cos() - point.y * rad.sin(),
        y: point.x * rad.sin() + point.y * rad.cos(),
        z: point.z,
        d: point.d
    }
}

fn rotate(point: &Point, r: &Point) -> Point
{
    rotate_x(&rotate_y(&rotate_z(point, r.z), r.y), r.x)
}

fn display(dpv: &Vec<Vec<DisplayPoint>>)
{
    for (y, v) in dpv.iter().enumerate() {
        for (x, p) in v.iter().enumerate() {
            put_at(x, y, p.d)
        }
    }
    std::io::stdout().flush().unwrap();
}

fn put_at(x: usize, y: usize, d: char)
{
    let esc = 27 as char;
    print!("{}[{};{}H", esc, y + 1, x + 1);
    print!("{}", d);
    print!("{}[1;1H", esc);
}

fn clear()
{
    let esc = 27 as char;
    print!("{}[2J{}[1;1H", esc, esc);
}

fn append_side(v: &mut Vec<Point>, d: &Side, c: char)
{
    v.append(
        &mut create_side(
            &Point {
                x: -SIZE/2f32,
                y: -SIZE/2f32,
                z: -SIZE/2f32,
                d: c 
            }
        ).iter_mut()
        .map(|p| match d { 
            Side::Top    => rotate_x(p,  PI/2f32),
            Side::Bottom => rotate_x(p, -PI/2f32),
            Side::Right  => rotate_y(p,  PI/2f32),
            Side::Left   => rotate_y(p, -PI/2f32),
            Side::Back   => rotate_y(p,  PI),
            _ => rotate_z(p, 0f32) 
        }).collect()
    );
}

fn init_display_points(dps: &mut Vec<Vec<DisplayPoint>>, offset: &Point)
{
    for _ in 0..((2f32 * SIZE + offset.y) as u32) {
        let mut tmp: Vec<DisplayPoint> = Vec::new();
        for _ in 0..((2f32 * SIZE + offset.x) as u32) {
            tmp.push(DisplayPoint { z: -1000f32, d: ' ' });
        }
        dps.push(tmp);
    }
}

fn prepare_display(dps: &mut Vec<Vec<DisplayPoint>>,
                   point: &Point, offset: &Point)
{
    let mut dpr: &mut DisplayPoint = dps 
        .get_mut((point.y + offset.y) as usize)
        .unwrap()
        .get_mut((point.x + offset.x) as usize)
        .unwrap();
    if point.z > dpr.z {
        dpr.z = point.z;
        dpr.d = point.d;
    }
}

fn main()
{
    let mut points: Vec<Point> = Vec::new();
    create_all_sides(&mut points);
    let offset = &Point {
        x: 5f32 + SIZE,
        y: SIZE,
        z: SIZE * 3f32,
        d: 0 as char 
    };
    let rotation: Point = Point {
        x: 0.03,
        y: 0.1,
        z: 0.04,
        d: 0 as char
    };
    loop {
        let mut display_points: Vec<Vec<DisplayPoint>> = Vec::new();
        init_display_points(&mut display_points, offset);
        for point in points.iter_mut() {
            prepare_display(&mut display_points, point, offset);
            *point = rotate(point, &rotation);
        }
        display(&display_points);
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
}
