// tells the rust compiler that this crate should not use rust's standard library except when explicitly told to.
#![cfg_attr(not(feature = "std"), no_std)]

mod demo;

#[cfg(test)]
mod tests;

// pub use demo::total::*;
pub use demo::store_struct::*;


