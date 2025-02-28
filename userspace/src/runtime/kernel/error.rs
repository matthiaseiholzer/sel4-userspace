use core::fmt::Debug;
use core::prelude::rust_2024::derive;

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
}
