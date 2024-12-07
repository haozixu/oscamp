#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::{print_with_color};

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    print_with_color!(32, "[WithColor]: Hello, Arceos!\n");
}
