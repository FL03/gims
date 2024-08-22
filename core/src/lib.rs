/*
    Appellation: gims-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Generalized Inventory Management System (G.I.M.S)
//!
//! A generalized inventory management system
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "gims_core"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::error::Error;

pub mod error;
pub mod traits;
pub mod types;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::error::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
