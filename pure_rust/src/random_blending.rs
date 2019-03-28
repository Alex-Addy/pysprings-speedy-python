use std::cmp::min;

use rand::prelude::*;
use rand::distributions::Uniform;

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

/// Calculate upper and lower bounds from index and distance.
///
/// Will not return a value lower than 0 or greater than max
fn calc_bounds(idx: usize, dist: usize, max: usize) -> (usize, usize) {
    let low = idx.saturating_sub(dist);
    let high = min(idx+dist, max);
    (low, high)
}

/// Randomly blend pixels from placing results into into
///
/// assumes from and into are the same size and are square
fn blend_into(square_size: usize, from: &Vec<Vec<u8>>, into: &mut Vec<Vec<u8>>) {
    let mut rng = thread_rng();
    for i in 0..from.len() {
        for j in 0..from.len() {
            //let blend_i = rng.gen_range(calc_bounds(i, square_size, from.len()));
            //let blend_j = rng.gen_range(calc_bounds(j, square_size, from.len()));

            let blend_i = rng.gen_range(i.saturating_sub(square_size), min(i+square_size, from.len()));
            let blend_j = rng.gen_range(j.saturating_sub(square_size), min(j+square_size, from.len()));
            into[i][j] = ((from[i][j] as u16 + from[blend_i][blend_j] as u16) / 2) as u8;
        }
    }
}


/// Generate a new grayscale image via random blending
pub fn random_blending(size: usize, iterations: usize, square_size: usize) -> Vec<Vec<u8>> {
    let mut first = gen_matrix(size);
    let mut second = first.clone();
    if iterations == 0 {
        // Not sure why this would happen but handling it here makes later easier
        return first;
    }
    if square_size == 0 || square_size == 1 {
        // If the square size is 0 or 1 no amount of iterations will change the output
        return first;
    }

    let mut swapped = false;

    for iter_num in 0..iterations {
        if swapped {
            blend_into(square_size, &second, &mut first);
            swapped = false;
        } else {
            blend_into(square_size, &first, &mut second);
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

