use core::default::Default;
use core::fmt;
use core::ops::Index;
use core::ops::{Deref, DerefMut};
use core::panic;
use core::ptr;
use core::slice;
use core::str;

#[derive(Clone, PartialEq, Eq)]
pub struct ArrayString<const C: usize> {
    data: [u8; C],
    len: usize,
}

impl<const C: usize> Default for ArrayString<C> {
    fn default() -> Self {
        ArrayString {
            data: [0; C],
            len: 0,
        }
    }
}

impl<const C: usize> Index<usize> for ArrayString<C> {
    type Output = u8;

    // Required method
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const C: usize> ArrayString<C> {
    pub fn push(&mut self, elem: char) {
        if self.len() < C {
            unsafe {
                self.push_unchecked(elem);
            }
        }
    }

    pub fn push_str(&mut self, s: &str) {
        if self.len + s.len() <= C {
            unsafe {
                let dst = self.data.as_mut_ptr().add(self.len());
                ptr::copy_nonoverlapping(s.as_ptr(), dst, s.len());
            };
            self.set_len(self.len() + s.len());
        } else {
            panic!();
        }
    }

    pub fn pop(&mut self) -> char {
        if self.len > 0 {
            let new_len = self.len() - 1;
            self.set_len(new_len);
            unsafe { ptr::read(self.data.as_ptr().add(new_len)) as char }
        } else {
            panic!();
        }
    }

    unsafe fn push_unchecked(&mut self, elem: char) {
        let len = self.len();
        ptr::write(self.data.as_mut_ptr().add(len), elem as u8);
        self.set_len(len + 1);
    }

    fn set_len(&mut self, len: usize) {
        self.len = len
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // /// Return a string slice of the whole `ArrayString`.
    // pub fn as_str(&self) -> &str {
    //     self
    // }

    //  /// Return a mutable string slice of the whole `ArrayString`.
    // pub fn as_mut_str(&mut self) -> &mut str {
    //     self
    // }

    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr() as *const u8
    }

    // fn as_mut_ptr(&mut self) -> *mut u8 {
    //     self.data.as_mut_ptr() as *mut u8
    // }

    pub fn from(s: &str) -> ArrayString<C> {
        let mut arraystr = Self::default();
        arraystr.push_str(s);
        arraystr
    }
}

impl<const C: usize> Deref for ArrayString<C> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe {
            let sl = slice::from_raw_parts(self.as_ptr(), self.len());
            str::from_utf8_unchecked(sl)
        }
    }
}

impl<const C: usize> DerefMut for ArrayString<C> {
    fn deref_mut(&mut self) -> &mut str {
        unsafe {
            let len = self.len();
            let sl = slice::from_raw_parts_mut(self.data.as_mut_ptr(), len);
            str::from_utf8_unchecked_mut(sl)
        }
    }
}

impl<const C: usize> fmt::Debug for ArrayString<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<const C: usize> fmt::Display for ArrayString<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

/// `Write` appends written data to the end of the string.
impl<const CAP: usize> fmt::Write for ArrayString<CAP> {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        fmt::Result::Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        fmt::Result::Ok(())
    }
}
