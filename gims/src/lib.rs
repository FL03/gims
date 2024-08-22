/*
    Appellation: gims <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Generalized Inventory Management System (G.I.M.S)
//!
//! A modern inventory management system designed to be used in a variety of applications.
//! The platform is designed to be modular and extensible, allowing for easy integration into
//! existing systems.
//!
//! ## Features
//!
//! - **Modular Design**: The system is designed to be modular, allowing for easy integration
//! into existing systems.
//!
//! - **Extensible**: Gims is easily extensible, allowing for the addition of new features
//! and functionality.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "gims"]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use gims_core::*;

pub mod prelude {
    pub use gims_core::prelude::*;
}
