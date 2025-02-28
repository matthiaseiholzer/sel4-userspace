use core::default::Default;

pub struct PageRights {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Default for PageRights {
    fn default() -> Self {
        PageRights {
            read: true,
            write: true,
            execute: true,
        }
    }
}
