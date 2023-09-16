use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Integer: {0}")]
    InvalidInteger(#[from] ParseIntError),
    #[error("Base64url: {0}")]
    Base64Id(#[from] base64id::Error),
}

pub enum Size {
    B64,
    B32,
    B16,
}

impl TryFrom<u8> for Size {
    type Error = &'static str;

    fn try_from(s: u8) -> Result<Self, Self::Error> {
        let s = match s {
            64 => Self::B64,
            32 => Self::B32,
            16 => Self::B16,
            _ => return Err("unsupported size. expected integer value 64, 32 or 16"),
        };

        Ok(s)
    }
}
