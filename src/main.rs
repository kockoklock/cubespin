use std::{f32::consts::PI, io::Write};

fn rotate_z(point: (f32, f32, f32), rad: f32) -> (f32, f32, f32)
{
    (
    point.0 * rad.cos() - point.1 * rad.sin(),
    point.0 * rad.sin() + point.1 * rad.cos(),
    point.2
    )
}

fn display(point: (f32, f32, f32), offset: (f32, f32, f32))
{
    let (x, y) = (
        (point.0 + offset.0) as i32,
        (point.1 + offset.1) as i32
    );
    put_at(x, y);
}

fn put_at(x: i32, y: i32)
{
    let (x, y) = (
        if x < 0 {0} else {x} as usize,
        if y < 0 {0} else {y} as usize
    );
    let esc = 27 as char;
    print!("{}[{};{}H", esc, y, x);
    print!("#");
    print!("{}[1;1H", esc);
    std::io::stdout().flush().unwrap();
}

fn clear()
{
    let esc = 27 as char;
    print!("{}[2J{}[1;1H", esc, esc);
}

fn main()
{
    let mut points: Vec<(f32,f32,f32)> = vec![
        (5f32, 5f32, 0f32),
        (-5f32, 5f32, 0f32),
        (-5f32, -5f32, 0f32),
        (5f32, -5f32, 0f32)
    ];
    let offset = (15f32, 15f32, 0f32);
    for _ in 1..200 {
        for point in points.iter_mut() {
            display(*point, offset);
            *point = rotate_z(*point, PI/32f32);
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        clear();
    };
}
