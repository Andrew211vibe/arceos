#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::{pinfo, pdev, pdebug};

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    pinfo!("Test info");
    pdev!("Test dev");
    pdebug!("Test debug");
    println!("Hello, world!");
}
