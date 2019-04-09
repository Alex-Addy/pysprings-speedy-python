use rand::distributions::Uniform;
use rand::prelude::*;

/// Generate a square matrix filled with noise
fn gen_matrix(size: usize) -> Vec<Vec<u8>> {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(0, 255);
    let mut src = distribution.sample_iter(&mut rng);
    let mut mat = vec![vec![0; size]; size];
    for row in mat.iter_mut() {
        for item in row.iter_mut() {
            *item = src.next().unwrap();
        }
    }

    mat
}

/// Randomly pick an index from PROBABILITIES based upon matrix weights.
fn pick_from_probability(probabilities: &[u8; 9]) -> usize {
    let mut rng = thread_rng();
    let total: usize = probabilities.iter().map(|x| *x as usize).sum();
    let pick = rng.gen_range(0, total);
    let mut choice_idx = 0;
    let mut sum = probabilities[choice_idx] as usize;
    while sum < pick {
        choice_idx += 1;
        sum += probabilities[choice_idx] as usize;
    }

    choice_idx
}

/// Convert a single dimensional index to the equivalent in two dimensions.
///
/// `idx` - the one dimensional index to convert
/// `row_len` - the length of each row in two dimensions
///
/// Assumes a x,y coordinate system where 0,0 is the center of the square.
///
/// # Example
///
/// ```
/// let (x, y) = split_index(2);
///
/// assert_eq!(x, -1);
/// assert_eq!(y, 1);
/// ```
fn split_index(idx: usize) -> (isize, isize) {
    match idx {
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

/// Pick an in bounds index to blend with.
///
/// Uses probability matrix PROBABILITIES to make pick.
fn pick_blend_indexes(i: usize, j: usize, size: usize, probabilities: &[u8; 9]) -> (usize, usize) {
    let i = i as isize;
    let j = j as isize;
    let size = size as isize;

    loop {
        let (row_diff, col_diff) = split_index(pick_from_probability(probabilities));
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
/// Assumes from and into are the same size and are square
fn blend_into(size: usize, from: &[Vec<u8>], into: &mut Vec<Vec<u8>>, probabilities: &[u8; 9]) {
    for i in 0..size {
        for j in 0..size {
            let (blend_i, blend_j) = pick_blend_indexes(i, j, size, probabilities);
            into[i][j] = ((u16::from(from[i][j]) + u16::from(from[blend_i][blend_j])) / 2) as u8;
        }
    }
}

/// Generate a new grayscale image via random blending
///
/// # Example
///
/// This will create a image that is 128x128 pixels with 20 iterations of
/// blending.
/// ```no-run
/// let grayscale = random_blending(128, 20);
/// ```
pub fn random_blending(size: usize, iterations: usize, probabilities: &[u8; 9]) -> Result<Vec<Vec<u8>>, String> {
    if iterations == 0 {
        return Err("Cannot perform zero iterations.".to_string());
    }
    if size <= 1 {
        return Err("Size of the image must be greater than 1 pixel".to_string());
    }

    // In order to make this efficient I will allocate only two arrays and then swap
    // back and forth which is going to be used as the source and destination.
    let mut first = gen_matrix(size);
    let mut second = first.clone();

    let mut swapped = false;

    for iter_num in 1..=iterations {
        if swapped {
            blend_into(size, &second, &mut first, probabilities);
            swapped = false;
        } else {
            blend_into(size, &first, &mut second, probabilities);
            swapped = true;
        }
        println!("Completed iteration {}/{}", iter_num, iterations);
    }

    // return the last matrix written to
    if swapped {
        Ok(second)
    } else {
        Ok(first)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Test the output of split index for all possible 3-size values.
    fn test_split_index_3() {
        // This test isn't very useful right now, but if multiple PROBABILITIES matrices
        // become supported it will be more useful.
        for i in 0..8 {
            let idxs = split_index(i);
            let exp_idxs = match i {
                0 => (-1, -1),
                1 => (-1, 0),
                2 => (-1, 1),
                3 => (0, -1),
                4 => (0, 0),
                5 => (0, 1),
                6 => (1, -1),
                7 => (1, 0),
                8 => (1, 1),
                _ => unreachable!(),
            };
            assert_eq!(exp_idxs, idxs);
        }
    }
}
