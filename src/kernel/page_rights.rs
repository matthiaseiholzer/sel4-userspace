use super::sys::types::seL4_CapRights;
use sel4_us::runtime::va_space::page_rights::PageRights;

impl Into<seL4_CapRights> for PageRights {
    fn into(self) -> seL4_CapRights {
        seL4_CapRights { words: [7; 1] }
    }
}
