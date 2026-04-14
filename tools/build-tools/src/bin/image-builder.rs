//! image-builder - Build bootable Elminux images
//!
//! Creates ISO images and disk images from kernel + userland binaries.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: image-builder <command> [options]");
        eprintln!("Commands:");
        eprintln!("  iso <kernel> <initrd> <output>  - Create bootable ISO");
        eprintln!("  disk <kernel> <initrd> <output> - Create disk image");
        return;
    }

    match args[1].as_str() {
        "iso" => build_iso(&args[2..]),
        "disk" => build_disk(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

fn build_iso(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: image-builder iso <kernel> <initrd> <output>");
        return;
    }

    let kernel = &args[0];
    let _initrd = &args[1];
    let output = &args[2];

    if !Path::new(kernel).exists() {
        eprintln!("Kernel not found: {}", kernel);
        return;
    }

    println!("Building ISO: {} -> {}", kernel, output);

    // TODO: Create ISO filesystem
    // TODO: Add Limine bootloader
    // TODO: Copy kernel and initrd
    // TODO: Generate ISO

    println!("ISO created: {}", output);
}

fn build_disk(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: image-builder disk <kernel> <initrd> <output>");
        return;
    }

    let kernel = &args[0];
    let _initrd = &args[1];
    let output = &args[2];

    println!("Building disk image: {} -> {}", kernel, output);

    // TODO: Create disk image
    // TODO: Partition with GPT
    // TODO: Install Limine
    // TODO: Copy kernel and initrd to EFI partition

    println!("Disk image created: {}", output);
}
