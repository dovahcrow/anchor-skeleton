use anchor_lang::prelude::*;
use std::convert::TryInto;

// Define errors, custom error code: 300 + idx => 0x12C + 0x${idx}
#[error(offset = 300)]
pub enum ErrorCode {
    #[msg("[T001] Contract address not correct")] //0x12C (300)
    ContractAddressNotCorrect,

    #[msg("[T001] The pool is suspended")] //0x13D (301)
    Suspended,
}

impl TryInto<ErrorCode> for u32 {
    type Error = (); // Error if u32 is out of range

    fn try_into(self) -> std::result::Result<ErrorCode, ()> {
        if (300..=301).contains(&self) {
            Ok(unsafe { std::mem::transmute(self - 300) })
        } else {
            Err(())
        }
    }
}
