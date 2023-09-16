use std::str::FromStr;

use base64id::{Id16, Id32, Id64};
use rand::random;

use crate::types::{
    Error,
    Size::{self, *},
};

pub fn encode(int: &str, size: Size, signed: bool) -> Result<String, Error> {
    let id = match (signed, size) {
        (true, size) => match size {
            B64 => Id64::from(i64::from_str(int)?).to_string(),
            B32 => Id32::from(i32::from_str(int)?).to_string(),
            B16 => Id16::from(i16::from_str(int)?).to_string(),
        },
        (false, size) => match size {
            B64 => Id64::from(u64::from_str(int)?).to_string(),
            B32 => Id32::from(u32::from_str(int)?).to_string(),
            B16 => Id16::from(u16::from_str(int)?).to_string(),
        },
    };

    Ok(id)
}

pub fn deocde(str: &str, size: Size, signed: bool) -> Result<String, Error> {
    let int = match (signed, size) {
        (true, size) => match size {
            B64 => i64::from(Id64::from_str(str)?).to_string(),
            B32 => i32::from(Id32::from_str(str)?).to_string(),
            B16 => i16::from(Id16::from_str(str)?).to_string(),
        },
        (false, size) => match size {
            B64 => u64::from(Id64::from_str(str)?).to_string(),
            B32 => u32::from(Id32::from_str(str)?).to_string(),
            B16 => u16::from(Id16::from_str(str)?).to_string(),
        },
    };

    Ok(int)
}

pub fn random_integer(size: Size, signed: bool) -> String {
    match (signed, size) {
        (true, size) => match size {
            B64 => i64::from(random::<Id64>()).to_string(),
            B32 => i32::from(random::<Id32>()).to_string(),
            B16 => i16::from(random::<Id16>()).to_string(),
        },
        (false, size) => match size {
            B64 => u64::from(random::<Id64>()).to_string(),
            B32 => u32::from(random::<Id32>()).to_string(),
            B16 => u16::from(random::<Id16>()).to_string(),
        },
    }
}
