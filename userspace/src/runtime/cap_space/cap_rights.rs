use core::clone::Clone;
use core::cmp::{Eq, PartialEq};
use core::default::Default;
use core::prelude::rust_2024::derive;

#[derive(PartialEq, Eq, Clone)]
pub struct CapRights {
    pub allow_grant_reply: bool,
    pub allow_grant: bool,
    pub allow_read: bool,
    pub allow_write: bool,
}

impl Default for CapRights {
    fn default() -> Self {
        CapRights {
            allow_grant_reply: true,
            allow_grant: true,
            allow_read: true,
            allow_write: true,
        }
    }
}

impl CapRights {
    pub fn new(
        allow_grant_reply: bool,
        allow_grant: bool,
        allow_read: bool,
        allow_write: bool,
    ) -> CapRights {
        CapRights {
            allow_grant_reply,
            allow_grant,
            allow_read,
            allow_write,
        }
    }

    pub const fn all_rights() -> CapRights {
        CapRights {
            allow_grant_reply: true,
            allow_grant: true,
            allow_read: true,
            allow_write: true,
        }
    }
}
