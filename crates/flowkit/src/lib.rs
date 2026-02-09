#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub mod corner;
pub mod curve;
pub mod edge;
pub mod extend_from;
pub mod path;
pub mod utils;
pub mod winding_order;

pub const CURVATURE: f32 = 0.25;
pub const OFFSET: f32 = 25.0;
