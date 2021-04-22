use libloading::Library;

use std::env;
use std::ffi::{CString, OsString};
use std::os::raw::c_char;
use std::path::Path;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn run() -> i32 {
    let argc = env::args_os().count();
    if argc != 2 {
        eprintln!(
            "Expected 1 argument, got {}. Usage: lv2_ttl_generator <path_to_lv2_binary>",
            argc.max(1) - 1
        );
        return EXIT_FAILURE;
    }

    let binary_path: OsString = env::args_os().last().unwrap();

    let binary = match unsafe { Library::new(&binary_path) } {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to load {:?}: {}", binary_path, e);
            return EXIT_FAILURE;
        }
    };

    let ttl_fn = {
        let symbol = unsafe {
            binary.get::<unsafe extern "C" fn(*const c_char) -> ()>(b"lv2_generate_ttl\0")
        };

        match symbol {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "Failed to load `void lv2_generate_ttl(const char *)`: {}",
                    e
                );
                return EXIT_FAILURE;
            }
        }
    };

    let file_stem: CString = {
        let stem = Path::new(&binary_path)
            .file_stem()
            .expect("The binary loaded successfully, but somehow it didn't have a file name.");

        let stem = match stem.to_str() {
            Some(s) => s,
            None => {
                eprintln!("Binary file name {:?} wasn't valid Unicode.", stem);
                return EXIT_FAILURE;
            }
        };

        CString::new(stem).expect("Couldn't make a C-style string out of the binary file name.")
    };

    unsafe {
        ttl_fn(file_stem.as_ptr());
    }

    EXIT_SUCCESS
}

fn main() {
    std::process::exit(run());
}
