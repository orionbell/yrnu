//! # yrnu
//! `yrnu` is a general purpose Rust crate for cyber specialiest and network administrators.
//! it provides countless utils for packet analysis, network device configurations and other utils
//! for automating network and cyber security tasks.


/// A module that provieds tools for heandling Ip and Mac addresses as well as tools to define
/// networks
#[warn(dead_code)]
#[warn(unused)]
pub mod address;
pub mod port;

// internal testing
#[cfg(test)]
mod tests {}
