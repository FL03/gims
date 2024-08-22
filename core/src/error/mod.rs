/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::error::Error;

pub(crate) mod error;

pub(crate) mod prelude {
    pub use super::error::Error;
}
