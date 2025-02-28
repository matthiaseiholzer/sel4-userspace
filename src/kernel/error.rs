use sel4_userspace::runtime::kernel::Error;

use super::sys::types::seL4_Error;

impl From<seL4_Error> for Error {
    fn from(value: seL4_Error) -> Self {
        match value {
            seL4_Error::seL4_InvalidArgument => Error::InvalidArgument,
            seL4_Error::seL4_InvalidCapability => Error::InvalidCapability,
            seL4_Error::seL4_IllegalOperation => Error::IllegalOperation,
            seL4_Error::seL4_RangeError => Error::RangeError,
            seL4_Error::seL4_AlignmentError => Error::AlignmentError,
            seL4_Error::seL4_FailedLookup => Error::FailedLookup,
            seL4_Error::seL4_TruncatedMessage => Error::TruncatedMessage,
            seL4_Error::seL4_DeleteFirst => Error::DeleteFirst,
            seL4_Error::seL4_RevokeFirst => Error::RevokeFirst,
            seL4_Error::seL4_NotEnoughMemory => Error::NotEnoughMemory,
            _ => panic!(),
        }
    }
}
