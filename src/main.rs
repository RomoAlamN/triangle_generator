mod wu_line;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        let num_triangles = usize::from_str_radix(&args[1], 10).unwrap();
        let num_not_triangles = usize::from_str_radix(&args[2], 10).unwrap();
        let triangles = generate_triangles(num_triangles);
        let not_triangles = generate_not_triangles(num_not_triangles);
    } else {
        println!("Usage: triangle_generator <#triangles> <#not-triangles>")
    }
}

use rand::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
//use std::fmt::format;

fn generate_not_triangles(num: usize) -> Vec<String> {
    let mut num_generated = 0;
    let mut rng = thread_rng();
    let mut paths = vec![];
    while num_generated < num {
        let mut matrix = [0u8; 32 * 32];
        let s_type = rng.gen_range(0.0..1.0);
        let status = if s_type < 0.333 {
            generate_rectangle(&mut matrix, &mut rng)
        } else if s_type < 0.666 {
            generate_circle(&mut matrix, &mut rng)
        } else {
            generate_line(&mut matrix, &mut rng)
        };
        if !status {
            continue;
        }
        num_generated += 1;
        let path_str = format!("./output/not_triangle_{num_generated}.png");
        paths.push(path_str.clone());
        let path = Path::new(&path_str);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 32, 32);
        encoder.set_color(png::ColorType::Grayscale); // black/white
        encoder.set_depth(png::BitDepth::Eight); // 0-256
        //        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let source_chromaticities = png::SourceChromaticities::new(
                // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&matrix).unwrap()
    }
    paths
}

fn generate_line(matrix: &mut [u8; 1024], rng: &mut ThreadRng) -> bool {
    let p1 = (rng.gen_range(0..32), rng.gen_range(0..32));
    let p2 = (rng.gen_range(0..32), rng.gen_range(0..32));
    draw_line::<1024, 32>(matrix, p1, p2);

    let dx = p2.0 as i32 - p1.0 as i32;
    let dy = p2.1 as i32 - p1.1 as i32;
    dx * dx + dy * dy >= 16
}
fn set_point(matrix: &mut [u8; 1024], x: usize, y: usize) {
    matrix[y * 32 + x] = 255;
}
fn generate_circle(matrix: &mut [u8; 1024], rng: &mut ThreadRng) -> bool {
    let r = rng.gen_range(4..16);
    let pos = (rng.gen_range(r..32 - r), rng.gen_range(r..32 - r));
    let mut d = (5 - r as i32 * 4) / 4;

    let mut x = 0;
    let mut y = r;

    loop {
        set_point(matrix, pos.0 + x, pos.1 + y);
        set_point(matrix, pos.0 + x, pos.1 - y);
        set_point(matrix, pos.0 - x, pos.1 + y);
        set_point(matrix, pos.0 - x, pos.1 - y);
        set_point(matrix, pos.0 + y, pos.1 + x);
        set_point(matrix, pos.0 + y, pos.1 - x);
        set_point(matrix, pos.0 - y, pos.1 + x);
        set_point(matrix, pos.0 - y, pos.1 - x);

        if d < 0 {
            d += 2 * x as i32 + 1;
        } else {
            d += 2 * (x as i32 - y as i32) + 1;
            y -= 1;
        }
        x += 1;
        if x > y {
            break;
        }
    }

    true
}

fn generate_rectangle(matrix: &mut [u8; 1024], rng: &mut ThreadRng) -> bool {
    let x = rng.gen_range(0..28);
    let y = rng.gen_range(0..28);
    let w = rng.gen_range(4..32-x);
    let h = rng.gen_range(4..32-y);

    draw_line::<1024, 32>(matrix, (x, y), (x + w, y));
    draw_line::<1024, 32>(matrix, (x, y + h), (x + w, y + h));
    draw_line::<1024, 32>(matrix, (x,y), (x, y + h));
    draw_line::<1024, 32>(matrix, (x + w, y), (x + w, y + h));

    true
}

fn generate_triangles(num: usize) -> Vec<String> {
    let mut num_generated = 0;
    let mut rng = thread_rng();
    let mut paths = vec![];
    while num_generated < num {
        let mut matrix = [0u8; 32 * 32];
        // naive triangle generator
        //        let p1 = (rng.gen_range(2..30), rng.gen_range(2..30));
        //        let p2 = (rng.gen_range(2..30), rng.gen_range(2..30));
        //        let p3 = (rng.gen_range(2..30), rng.gen_range(2..30));

        let first_x = rng.gen_range(0..31);
        let first_y = rng.gen_range(0..31);

        let p1 = (
            (first_x + rng.gen_range(4..16)) % 32,
            (first_y + rng.gen_range(4..16)) % 32,
        );
        let p2 = (
            (first_x + rng.gen_range(16..28)) % 32,
            (first_y + rng.gen_range(16..28)) % 32,
        );
        let p3 = (first_x, first_y);

        draw_line::<1024, 32>(&mut matrix, p1, p2);
        draw_line::<1024, 32>(&mut matrix, p2, p3);
        draw_line::<1024, 32>(&mut matrix, p3, p1);

        if get_angles(p1, p2, p3).0 > 0.4 {
            //not a good triangle
            continue;
        }
        num_generated += 1;
        let path_str = format!("./output/triangle_{num_generated}.png");
        paths.push(path_str.clone());
        let path = Path::new(&path_str);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 32, 32);
        encoder.set_color(png::ColorType::Grayscale); // black/white
        encoder.set_depth(png::BitDepth::Eight); // 0-256
                                                 //        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&matrix).unwrap()
    }
    paths
}

fn get_angles(p1: (usize, usize), p2: (usize, usize), p3: (usize, usize)) -> (f32, f32) {
    let x0 = p1.0 as i32;
    let x1 = p2.0 as i32;
    let x2 = p3.0 as i32;
    let y0 = p1.1 as i32;
    let y1 = p2.1 as i32;
    let y2 = p3.1 as i32;

    let slope1 = get_slope((x0, y0), (x1, y1));
    let slope2 = get_slope((x1, y1), (x2, y2));
    let slope3 = get_slope((x2, y2), (x0, y0));

    let theta1 = (slope1 / slope2).atan().abs();
    let theta2 = (slope2 / slope3).atan().abs();
    let theta3 = (slope3 / slope1).atan().abs();

    (
        f32::min(f32::min(theta1, theta2), theta3),
        f32::max(f32::max(theta1, theta2), theta3),
    )
}
fn get_slope(p1: (i32, i32), p2: (i32, i32)) -> f32 {
    (p2.1 - p1.1) as f32 / (p2.0 - p1.0) as f32
}

fn draw_line<const SIZE: usize, const X_SIZE: usize>(
    matrix: &mut [u8; SIZE],
    p1: (usize, usize),
    p2: (usize, usize),
) {
    wu_line::draw_line::<SIZE, X_SIZE>(matrix, p1, p2)
}
