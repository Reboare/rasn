//! Version 2, Community Version (RFC 1901)
use rasn::{
    types::{Integer, OctetString},
    AsnType, Decode, Encode,
};

#[derive(AsnType, Debug, Clone, Decode, Encode)]
pub struct Message<T> {
    pub version: Integer,
    pub community: OctetString,
    pub data: T,
}

impl<T> Message<T> {
    pub const VERSION: u64 = 1;
}
