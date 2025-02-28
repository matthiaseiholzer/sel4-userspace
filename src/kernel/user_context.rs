use super::sys::types::seL4_Word;

#[repr(C)]
#[derive(Default)]
pub struct seL4_UserContext {
    pub pc: seL4_Word,
    pub ra: seL4_Word,
    pub sp: seL4_Word,
    pub gp: seL4_Word,

    pub s0: seL4_Word,
    pub s1: seL4_Word,
    pub s2: seL4_Word,
    pub s3: seL4_Word,
    pub s4: seL4_Word,
    pub s5: seL4_Word,
    pub s6: seL4_Word,
    pub s7: seL4_Word,
    pub s8: seL4_Word,
    pub s9: seL4_Word,
    pub s10: seL4_Word,
    pub s11: seL4_Word,

    pub a0: seL4_Word,
    pub a1: seL4_Word,
    pub a2: seL4_Word,
    pub a3: seL4_Word,
    pub a4: seL4_Word,
    pub a5: seL4_Word,
    pub a6: seL4_Word,
    pub a7: seL4_Word,

    pub t0: seL4_Word,
    pub t1: seL4_Word,
    pub t2: seL4_Word,
    pub t3: seL4_Word,
    pub t4: seL4_Word,
    pub t5: seL4_Word,
    pub t6: seL4_Word,

    pub tp: seL4_Word,
}
