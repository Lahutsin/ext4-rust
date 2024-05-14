extern crate libc;

use libc::{open, write, ftruncate, O_CREAT, O_RDWR, O_APPEND};
use std::ffi::CString;
use std::io;

fn write_data_low_level(file_path: &str, data: &[u8]) -> io::Result<()> {
    let c_file_path = CString::new(file_path).unwrap();
    let fd = unsafe { open(c_file_path.as_ptr(), O_CREAT | O_RDWR, 0o644) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    let result = unsafe { write(fd, data.as_ptr() as *const libc::c_void, data.len()) };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

fn truncate_file_low_level(file_path: &str, size: u64) -> io::Result<()> {
    let c_file_path = CString::new(file_path).unwrap();
    let fd = unsafe { open(c_file_path.as_ptr(), O_RDWR, 0o644) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    let result = unsafe { ftruncate(fd, size as libc::off_t) };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

/*
fn main() -> io::Result<()> {
    let file_path = "example_low_level.txt";
    
    // Low-level writing of data to a file
    write_data_low_level(file_path, b"Hello, world!")?;
    
    // Truncating a file to 5 bytes in a low-level way
    truncate_file_low_level(file_path, 5)?;

    Ok(())
}
