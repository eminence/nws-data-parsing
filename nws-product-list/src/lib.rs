//! NWS Text Products
//!
//! This crate contains a large enum for each [NWS text product](https://forecast.weather.gov/product_types.php).
//!
//! Last updated May 15, 2022
//!
//! This crate uses data published from NWS, but is otherwise unaffiliated with the National Weather Service,
//! and is not an official NWS library.

mod gen;
pub use gen::NWSProduct;
