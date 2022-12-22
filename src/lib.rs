extern crate libc;

use std::ffi::CStr;
use std::str;
use libc::{getmntent, c_char, c_int, FILE};

#[repr(C)]
pub struct Mntent {
    mnt_fsname: *const c_char,
    mnt_dir: *const c_char,
    mnt_type: *const c_char,
    mnt_opts: *const c_char,
    mnt_freq: c_int,
    mnt_passno: c_int,
}

pub fn get_mounted_disks() -> Vec<(String, String, String)> {
    let mut disks: Vec<(String, String, String)> = Vec::new();

    // Open the mounted file systems file
    let mount_file = unsafe {
        let file = libc::fopen("/etc/mnttab", "r\0".as_ptr() as *const c_char);
        std::fs::File::from_raw_fd(libc::fileno(file))
    };
    let reader = std::io::BufReader::new(mount_file);

    // Iterate over the lines in the file
    let mut entry: *mut Mntent = std::ptr::null_mut();
    let mut file: *mut FILE = std::ptr::null_mut();
    while unsafe {
        entry = getmntent(file);
        !entry.is_null()
    } {
        let fsname = unsafe { CStr::from_ptr((*entry).mnt_fsname) };
        let dir = unsafe { CStr::from_ptr((*entry).mnt_dir) };
        let mnt_type = unsafe { CStr::from_ptr((*entry).mnt_type) };

        disks.push((
            fsname.to_str().unwrap().to_string(),
        dir.to_str().unwrap().to_string(),
        mnt_type.to_str().unwrap().to_string(),
    ));
    }

    disks
}
