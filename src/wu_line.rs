pub fn draw_line<const SIZE: usize, const X_SIZE: usize>(
    matrix: &mut [u8; SIZE],
    p1: (usize, usize),
    p2: (usize, usize),
) {
    let x0: i32 = p1.0.try_into().unwrap();
    let y0: i32 = p1.1.try_into().unwrap();
    let x1: i32 = p2.0.try_into().unwrap();
    let y1: i32 = p2.1.try_into().unwrap();

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut cur_x = x0;
    let mut cur_y = y0;
    loop {
        matrix[cur_y as usize * X_SIZE + cur_x as usize] = 255;
        if cur_x == x1 && cur_y == y1 {
            break;
        }
        if err * 2 > -dx {
            err -= dy;
            cur_x += sx;
        } else if err * 2 < dy {
            err += dx;
            cur_y += sy;
        }
    }
}
