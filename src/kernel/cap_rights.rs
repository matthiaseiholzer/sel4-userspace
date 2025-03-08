use super::sys::types::seL4_CapRights;
use sel4_us::runtime::cap_space::cap_rights::CapRights;

impl From<seL4_CapRights> for CapRights {
    fn from(cr: seL4_CapRights) -> CapRights {
        let rights = cr.words[0];
        let allow_gran_reply = rights & (1 << 3) == (1 << 3);
        let allow_grant = rights & (1 << 2) == (1 << 2);
        let allow_read = rights & (1 << 1) == (1 << 1);
        let allow_write = rights & (1 << 0) == 1;
        CapRights::new(allow_gran_reply, allow_grant, allow_read, allow_write)
    }
}

impl Into<seL4_CapRights> for CapRights {
    fn into(self) -> seL4_CapRights {
        let rights: u64 = (self.allow_grant_reply as u64) << 3
            | (self.allow_grant as u64) << 2
            | (self.allow_read as u64) << 1
            | (self.allow_write as u64) << 0;

        seL4_CapRights { words: [rights; 1] }
    }
}
