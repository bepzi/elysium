use core::pin::Pin;
use ffi::*;
use rand::Rng;
use std::ptr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wrapper.hpp");

        type AudioBufferF32;

        fn get_num_channels(buf: &AudioBufferF32) -> i32;

        fn get_num_samples(buf: &AudioBufferF32) -> i32;

        fn get_array_of_read_pointers(buf: &AudioBufferF32) -> *const *const f32;

        fn get_array_of_write_pointers(buf: Pin<&mut AudioBufferF32>) -> *mut *mut f32;

        fn clear(buf: Pin<&mut AudioBufferF32>);
    }

    extern "Rust" {
        fn process_block(buf: Pin<&mut AudioBufferF32>, sample_rate: f64);
    }
}

fn process_block(buf: Pin<&mut AudioBufferF32>, sample_rate: f64) {
    let channels = {
        let count = get_num_channels(&buf);
        if count < 0 {
            0
        } else {
            count as usize
        }
    };

    let samples = {
        let count = get_num_samples(&buf);
        if count < 0 {
            0
        } else {
            count as usize
        }
    };

    let mut rng = rand::thread_rng();
    let raw_array = get_array_of_write_pointers(buf);

    for i in 0..channels {
        unsafe {
            let raw_channel = *raw_array.add(i);
            for j in 0..samples {
                let sample = (rng.gen::<f32>() * 2.0) - 1.0;
                ptr::write(raw_channel.add(j), sample * 0.2);
            }
        }
    }
}
