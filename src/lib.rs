//! # Yrnu
//! `yrnu` is a general purpose Rust crate for cyber specialist and network administrators.
//! it provides countless utils for packet analysis, network device configurations and other utils
//! for automating network and cyber security tasks.

pub mod config;
/// A module that provides tools for handling IP and MAC addresses as well as tools to define
/// networks
#[warn(unused)]
pub mod core;
pub mod error;
pub mod lua;
pub mod packet;
pub mod parser;
pub mod port;
// internal testing
#[cfg(test)]
mod tests {}
