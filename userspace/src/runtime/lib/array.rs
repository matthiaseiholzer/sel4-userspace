use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::default::Default;
use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::ops::Index;
use core::ops::IndexMut;
use core::prelude::rust_2024::derive;
use core::ptr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array<T, const C: usize> {
    pub data: [T; C],
    pub len: usize,
}

impl<T, const C: usize> Default for Array<T, C> {
    fn default() -> Self {
        unsafe {
            Array {
                data: MaybeUninit::uninit().assume_init(),
                len: 0,
            }
        }
    }
}

impl<T, const C: usize> Index<usize> for Array<T, C> {
    type Output = T;

    // Required method
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const C: usize> IndexMut<usize> for Array<T, C> {
    // Required method
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const C: usize> Array<T, C> {
    pub fn push(&mut self, elem: T) {
        if self.len < C {
            unsafe {
                self.push_unchecked(elem);
            }
        }
    }

    pub fn pop(&mut self) -> T {
        let new_len = self.len - 1;
        self.len = new_len;
        unsafe { ptr::read(self.data.as_ptr().add(new_len)) }
    }

    unsafe fn push_unchecked(&mut self, elem: T) {
        let len = self.len;
        ptr::write(self.data.as_mut_ptr().add(len), elem);
        self.len += 1;
    }
}
