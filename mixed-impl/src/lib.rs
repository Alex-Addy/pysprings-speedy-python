use std::slice;
use std::os::raw::c_char;
use std::ffi::CStr;

use image::{ImageBuffer, Luma};

mod random_blending;

fn convert_slice_to_prob_arr(sli: &[u8]) -> Option<[u8; 9]> {
    if sli.len() != 9 {
        None
    } else {
        Some([sli[0], sli[1], sli[2],
             sli[3], sli[4], sli[5],
             sli[6], sli[7], sli[8]])
    }
}

#[no_mangle]
/// Perform blending with the given arguments and output the results to the given path
pub extern "C" fn random_blending_c(
    size: i32,
    iterations: i32,
    prob_ptr: *const i32,
    out_file_ptr: *const c_char) -> i32 {

    // enforce parameter values
    if prob_ptr.is_null() || out_file_ptr.is_null() {
        println!("one of your pointers is null");
        return -1;
    }
    if size <= 0 {
        println!("size must be greater than zero");
        return -1;
    }
    if iterations <= 0 {
        println!("iterations must be greater than zero");
        return -1;
    }

    // convert probability array
    let prob_tmp_slice = unsafe {
        // NEVER specify the length like this in real code
        // ALWAYS pass in the length of the array from python
        // If array is not 9 long this will invoke undefined behavior
        slice::from_raw_parts(prob_ptr, 9)
    };
    let probabilities = prob_tmp_slice.iter().map(|x| *x as u8).collect::<Vec<u8>>();
    let prob_arr = if let Some(arr) = convert_slice_to_prob_arr(&probabilities) {
        arr
    } else {
        println!("This shouldn't happen but I got a probablity array that is not a length of 9");
        return -1;
    };

    // convert file path to useable string
    let c_str = unsafe { CStr::from_ptr(out_file_ptr) };
    let out_path = if let Ok(s) = c_str.to_str() {
        s
    } else {
        println!("could not convert out_file_ptr into valid utf-8 string");
        return -1;
    };

    // end of ffi handling

    let mut img = ImageBuffer::new(size as u32, size as u32);
    let grayscale = match random_blending::random_blending(size as usize, iterations as usize, &prob_arr) {
        Ok(grayscale) => grayscale,
        Err(err) => {
            println!("Error while blending: {}", err);
            return -1;
        }
    };
    println!("Converting raw to image");
    for (x, row) in grayscale.into_iter().enumerate() {
        for (y, c) in row.into_iter().enumerate() {
            let pixel = Luma([c]);
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }
    println!("Done converting");

    // we cannot just unwrap the result here as panicing across ffi boundaries is undefined
    // behavior
    match img.save(out_path) {
        Err(err) => {
            println!("Error while saving image: {}", err);
            return -1;
        },
        Ok(_) => {
            return 0;
        },
    }
}

