#[macro_use]
extern crate criterion;
mod buddy_allocator;

criterion_main!(buddy_allocator::bench,);
