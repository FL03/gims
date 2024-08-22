/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, strum::EnumDiscriminants, strum::EnumIs)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "PascalCase"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(
    name(Errors),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantArray,
        strum::VariantNames,
    ),
    strum(serialize_all = "PascalCase")
)]
pub enum Error {
    ConvertError(String),
    Unkown(String),
}

impl Error {
    pub fn unknown(s: &str) -> Self {
        Error::Unkown(s.to_string())
    }

    pub fn kind(&self) -> Errors {
        match self {
            Error::ConvertError(_) => Errors::ConvertError,
            Error::Unkown(_) => Errors::Unkown,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Error::ConvertError(s) => s,
            Error::Unkown(s) => s,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "[{}] {}", self.kind(), self.message())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "[{}] {}", self.kind(), self.message())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unkown(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unkown(s)
    }
}

impl From<(Errors, String)> for Error {
    fn from((kind, message): (Errors, String)) -> Self {
        match kind {
            Errors::ConvertError => Error::ConvertError(message),
            Errors::Unkown => Error::Unkown(message),
        }
    }
}
