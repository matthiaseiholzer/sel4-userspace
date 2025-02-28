use core::clone::Clone;
use core::cmp::{Eq, PartialEq};
use core::prelude::rust_2024::derive;

#[derive(PartialEq, Eq, Clone)]
pub enum Attributes {
    Default,
    ExecuteNever,
}
