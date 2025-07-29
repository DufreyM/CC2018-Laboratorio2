use minifb::{Key, Window, WindowOptions};
use std::{thread, time::Duration};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 5; 

type Grid = [[bool; WIDTH]; HEIGHT];

fn draw(buffer: &mut Vec<u32>, grid: &Grid) {
    let dead_color = 0x1B1036; // púrpura oscuro
    let live_colors = [
        0xFFFF00, 
        0x00FFAA, 
        0xFF00FF,
        0x00FFFF, 
        0xFFAA00,
        0xAAFF00, 
        0xFF6666, 
        0x66AAFF,
    ];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if grid[y][x] {
                let hash = (x + y) % live_colors.len();
                live_colors[hash]
            } else {
                dead_color
            };
            point(buffer, x, y, color);
        }
    }
}

fn count_neighbors(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < WIDTH && (ny as usize) < HEIGHT {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn step(current: &Grid) -> Grid {
    let mut next = [[false; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = current[y][x];
            let neighbors = count_neighbors(current, x, y);
            next[y][x] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    next
}

fn point(buffer: &mut Vec<u32>, x: usize, y: usize, color: u32) {
    for dy in 0..SCALE {
        for dx in 0..SCALE {
            let px = x * SCALE + dx;
            let py = y * SCALE + dy;
            if px < WIDTH * SCALE && py < HEIGHT * SCALE {
                buffer[py * WIDTH * SCALE + px] = color;
            }
        }
    }
}


fn add_glider(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_blinker(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [(0, 0), (1, 0), (2, 0)];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_block(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_beehive(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (1, 0), (2, 0),
        (0, 1),       (3, 1),
        (1, 2), (2, 2),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_loaf(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
                (1, 0), (2, 0),
        (0, 1),        (3, 1),
        (1, 2),      (3, 2),
            (2, 3),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_boat(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1),       (2, 1),
              (1, 2),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_toad(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_beacon(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1),
                    (3, 2),
        (2, 3), (3, 3),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn add_lwss(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1),
        (0, 2),       (4, 2),
              (1, 3),       (4, 3),
    ];
    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}


fn setup_initial_pattern(grid: &mut Grid) {
    add_block(grid, 4, 4);
    add_beehive(grid, 16, 4);
    add_loaf(grid, 30, 4);
    add_boat(grid, 45, 4);

    add_blinker(grid, 4, 20);
    add_toad(grid, 16, 20);
    add_beacon(grid, 30, 20);

    add_glider(grid, 4, 36);
    add_lwss(grid, 16, 36);

    add_glider(grid, 60, 15);
    add_block(grid, 70, 25);
    add_lwss(grid, 50, 50);
    add_blinker(grid, 40, 10);
    add_beacon(grid, 20, 50);
    add_boat(grid, 35, 30);
    add_to_random_positions(grid);
}

fn add_to_random_positions(grid: &mut Grid) {
    add_glider(grid, 80, 10);
    add_block(grid, 75, 15);
    add_beehive(grid, 65, 35);
    add_blinker(grid, 55, 40);
    add_loaf(grid, 25, 60);
    add_boat(grid, 15, 70);
    add_beacon(grid, 45, 60);
    add_toad(grid, 70, 70);
    add_lwss(grid, 5, 90);
}

fn add_glider_gun(grid: &mut Grid, x: usize, y: usize) {
    let pattern = [
        (0, 4), (0, 5), (1, 4), (1, 5),

        (10, 4), (10, 5), (10, 6),
        (11, 3), (11, 7),
        (12, 2), (12, 8),
        (13, 2), (13, 8),
        (14, 5),
        (15, 3), (15, 7),
        (16, 4), (16, 5), (16, 6),
        (17, 5),

        (20, 2), (20, 3), (20, 4),
        (21, 2), (21, 3), (21, 4),
        (22, 1), (22, 5),
        (24, 0), (24, 1), (24, 5), (24, 6),

        (34, 2), (34, 3),
        (35, 2), (35, 3),
    ];

    for (dx, dy) in pattern {
        if x + dx < WIDTH && y + dy < HEIGHT {
            grid[y + dy][x + dx] = true;
        }
    }
}

fn fill_more_patterns(grid: &mut Grid) {
    add_toad(grid, 17, 72);
    add_glider(grid, 10, 49);
    add_loaf(grid, 72, 51);
    add_toad(grid, 16, 43);
    add_beacon(grid, 32, 57);
    add_glider(grid, 32, 81);
    add_beehive(grid, 68, 11);
    add_glider(grid, 92, 79);
    add_beacon(grid, 77, 25);
    add_block(grid, 94, 55);
    add_beacon(grid, 57, 16);
    add_blinker(grid, 49, 52);
    add_beehive(grid, 35, 50);
    add_block(grid, 7, 34);
    add_boat(grid, 73, 78);
    add_loaf(grid, 85, 24);
    add_beehive(grid, 27, 43);
    add_boat(grid, 7, 8);
    add_glider(grid, 38, 82);
    add_loaf(grid, 89, 32);
    add_lwss(grid, 44, 40);
    add_loaf(grid, 3, 81);
    add_blinker(grid, 95, 16);
    add_block(grid, 53, 23);
    add_loaf(grid, 59, 14);
    add_lwss(grid, 19, 27);
    add_loaf(grid, 34, 54);
    add_blinker(grid, 16, 24);
    add_beehive(grid, 4, 32);
    add_blinker(grid, 47, 8);
    add_toad(grid, 28, 72);
    add_boat(grid, 68, 52);
    add_block(grid, 70, 46);
    add_beehive(grid, 47, 45);
    add_beehive(grid, 79, 9);
    add_boat(grid, 8, 35);
    add_loaf(grid, 56, 14);
    add_boat(grid, 91, 16);
    add_beehive(grid, 55, 31);
    add_boat(grid, 9, 49);
    add_boat(grid, 6, 69);
    add_loaf(grid, 87, 19);
    add_beacon(grid, 48, 88);
    add_loaf(grid, 1, 25);
    add_beehive(grid, 14, 7);
    add_boat(grid, 76, 81);
    add_toad(grid, 10, 27);
    add_boat(grid, 90, 42);
    add_lwss(grid, 83, 70);
    add_block(grid, 31, 23);
    add_blinker(grid, 49, 19);
    add_beehive(grid, 10, 24);
    add_blinker(grid, 35, 67);
    add_beehive(grid, 92, 76);
    add_lwss(grid, 77, 26);
    add_blinker(grid, 95, 79);
    add_beehive(grid, 7, 39);
    add_glider(grid, 55, 70);
    add_beacon(grid, 78, 62);
    add_loaf(grid, 89, 54);
}

fn main() {
    let mut window = Window::new(
        "Game of Life - Rust + minifb",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    ).unwrap();

    let mut buffer = vec![0; WIDTH * HEIGHT * SCALE * SCALE];
    let mut grid = [[false; WIDTH]; HEIGHT];

    setup_initial_pattern(&mut grid);
    fill_more_patterns(&mut grid);
    add_to_random_positions(&mut grid);
    add_glider_gun(&mut grid, 10, 60);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw(&mut buffer, &grid);
        window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE).unwrap();
        grid = step(&grid);
        thread::sleep(Duration::from_millis(100));
    }
}

