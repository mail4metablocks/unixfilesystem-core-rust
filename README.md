
# Mounted Disks Crate

This crate provides a simple interface for getting information on mounted disks on Unix systems.
Example

```
extern crate mounted_disks;

fn main() {
    let disks = mounted_disks::get_mounted_disks();
    for (fsname, dir, mnt_type) in disks {
        println!("File system: {}", fsname);
        println!("Mount point: {}", dir);
        println!("File system type: {}", mnt_type);
    }
}

Installation

To use this crate in your project, add the following dependency to your Cargo.toml file:


[dependencies]
mounted_disks = "0.1"

```
License

This crate is licensed under the Apache license. See the LICENSE file for details.
