use rand::distributions::Uniform;
use rand::prelude::*;

const PROBABILITIES: &[usize; 9] = &[1, 4, 5, 1, 0, 5, 1, 1, 1];

/// Generate a square matrix filled with noise
fn gen_matrix(size: usize) -> Vec<Vec<u8>> {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(0, 255);
    let mut src = distribution.sample_iter(&mut rng);
    let mut mat = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            mat[i][j] = src.next().unwrap();
        }
    }

    mat
}

fn pick_from_probability() -> usize {
    let mut rng = thread_rng();
    let total: usize = PROBABILITIES.iter().sum();
    let pick = rng.gen_range(0, total);
    let mut choice_idx = 0;
    let mut sum = PROBABILITIES[choice_idx];
    while sum < pick {
        choice_idx += 1;
        sum += PROBABILITIES[choice_idx];
    }

    choice_idx
}

fn convert_pick_to_rel_idx(pick: usize) -> (isize, isize) {
    // TODO make this general
    match pick {
        0 => (-1, -1),
        1 => (-1, 0),
        2 => (-1, 1),
        3 => (0, -1),
        4 => (0, 0),
        5 => (0, 1),
        6 => (1, -1),
        7 => (1, 0),
        8 => (1, 1),
        x => panic!("Got pick with impossible value of {}", x),
    }
}

fn get_idx(i: usize, j: usize, size: usize) -> (usize, usize) {
    let i = i as isize;
    let j = j as isize;
    let size = size as isize;

    loop {
        let (row_diff, col_diff) = convert_pick_to_rel_idx(pick_from_probability());
        // ensure row_diff is in range
        if row_diff + i >= size || row_diff + i < 0 {
            continue;
        }
        // ensure col_diff is in range
        if col_diff + j >= size || col_diff + j < 0 {
            continue;
        }
        return ((row_diff + i) as usize, (col_diff + j) as usize);
    }
}

/// Randomly blend pixels from placing results into into
///
/// assumes from and into are the same size and are square
fn blend_into(size: usize, from: &Vec<Vec<u8>>, into: &mut Vec<Vec<u8>>) {
    for i in 0..from.len() {
        for j in 0..from.len() {
            let (blend_i, blend_j) = get_idx(i, j, size);
            into[i][j] = ((from[i][j] as u16 + from[blend_i][blend_j] as u16) / 2) as u8;
        }
    }
}

/// Generate a new grayscale image via random blending
pub fn random_blending(size: usize, iterations: usize) -> Vec<Vec<u8>> {
    let mut first = gen_matrix(size);
    let mut second = first.clone();
    if iterations == 0 {
        // Not sure why this would happen but handling it here makes later easier
        return first;
    }
    if size == 0 || size == 1 {
        // If the square size is 0 or 1 no amount of iterations will change the output
        return first;
    }

    let mut swapped = false;

    for iter_num in 1..=iterations {
        if swapped {
            blend_into(size, &second, &mut first);
            swapped = false;
        } else {
            blend_into(size, &first, &mut second);
            swapped = true;
        }
        println!("Completed iteration {}/{}", iter_num, iterations);
    }

    // pick which matrix to return
    if swapped {
        second
    } else {
        first
    }
}
