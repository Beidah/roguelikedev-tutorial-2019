use super::tile::Tile;
use rand::distributions::Uniform;
use rand::Rng;
use std::collections::HashSet;


pub fn make_cellular_cave(width: usize, height: usize) -> Vec<Tile> {
    loop {
        // Set up RNG
        let dist = Uniform::new(0, 100);
        let mut rng = rand::thread_rng();

        // Initialize buffers
        let len = width * height;
        let mut buffer_a = Vec::with_capacity(len);
        let mut buffer_b = Vec::with_capacity(len);
        buffer_b.resize(len, Tile::Ground);

        // Set up buffer_a with a 40% spawn rate
        for _ in 0..len {
            if rng.sample(dist) <= 40 {
                buffer_a.push(Tile::Wall);
            } else {
                buffer_a.push(Tile::Ground);
            }
        }

        cave_copy(&mut buffer_b, &mut buffer_a, width);
        cave_copy(&mut buffer_a, &mut buffer_b, width);
        cave_copy(&mut buffer_b, &mut buffer_a, width);
        cave_copy(&mut buffer_a, &mut buffer_b, width);
        cave_copy(&mut buffer_b, &mut buffer_a, width);

        let copied_count = flood_copy(&mut buffer_a, &buffer_b, width, height);
        println!("Copy count: {}", copied_count);
        if copied_count >= (width * height) as i32 / 2 {
            println!("Map complete");
            return buffer_a;
        }
    }
}

fn cave_copy(dest: &mut Vec<Tile>, src: &Vec<Tile>, width: usize) {
    for (i, tile) in dest.iter_mut().enumerate() {
        if range_count(src, i, width, 1) >= 5 || range_count(src, i, width, 2) <= 1 {
            *tile = Tile::Wall;
        } else {
            *tile = Tile::Ground;
        }
    }
}

fn flood_copy(dest: &mut Vec<Tile>, src: &Vec<Tile>, width: usize, height: usize) -> i32 {
    dest.iter_mut().map(|x| *x = Tile::Wall).count();
    let mut copied_count = 0;
    let start = {
        let mut rng = rand::thread_rng();
        let d_width = Uniform::from(0..=width);
        let d_height = Uniform::from(0..height);
        let mut x = rng.sample(d_width);
        let mut y = rng.sample(d_height);
        let mut tries = 0;
        while src.get(y * width + x) == Some(&Tile::Wall) {
            x = rng.sample(d_width);
            y = rng.sample(d_height);
            tries += 1;
            if tries > 100 {
                return 0;
            }
        }

        (x, y)
    };

    let mut open_set = HashSet::new();
    let mut closed_set = HashSet::new();
    open_set.insert(start);
    while !open_set.is_empty() {
        let loc: (usize, usize) = *open_set.iter().next().unwrap();
        open_set.remove(&loc);
        if closed_set.contains(&loc) {
            continue;
        } else {
            closed_set.insert(loc);
        }

        match src.get(loc.1 * width + loc.0) {
            Some(Tile::Ground) => {
                dest[loc.1 * width + loc.0] = Tile::Ground;
                copied_count += 1;
                if loc.0 > 1 {
                    open_set.insert((loc.0 - 1, loc.1));
                }
                if loc.0 < (width - 2) {
                    open_set.insert((loc.0 + 1, loc.1));
                }
                if loc.1 > 1 {
                    open_set.insert((loc.0, loc.1 - 1));
                }
                if loc.1 < (height - 2) {
                    open_set.insert((loc.0, loc.1 + 1));
                }
            }
            _ => {}
        }
    }

    copied_count
}

fn range_count(buf: &Vec<Tile>, i: usize, width: usize, range: usize) -> u32 {
    let mut total = 0;
    let (x, y) = (i % width, i / width);

    for y in ((y as isize - range as isize) as usize)..=(y + range as usize) {
        for x in ((x as isize - range as isize) as usize)..=(x + range as usize) {
            if y == 0 && x == 0 {
                continue;
            } else {
                match buf.get(y * width + x) {
                    Some(Tile::Ground) => {}
                    _ => {
                        total += 1;
                    }
                }
            }

        }

    }

    total
}