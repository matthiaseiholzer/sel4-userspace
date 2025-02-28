use criterion::{criterion_group, Criterion};

use rand::{random, Rng};

criterion_group!(bench, bench_list, bench_tree);

fn bench_list(bench: &mut Criterion) {
    use page_allocator::buddy_allocator_list::BuddyAllocator;

    bench.bench_function("bench_list", move |bh| {
        const SIZE_BITS: u8 = 21;

        let memory: Vec<u8> = Vec::with_capacity((1 << SIZE_BITS) * BuddyAllocator::PAGE_SIZE);

        let mut allocs: Vec<(usize, u8)> = vec![];
        let mut rng = rand::thread_rng();
        let addr = memory.as_ptr() as usize;

        let mut allocator = BuddyAllocator::new(addr, SIZE_BITS);

        bh.iter(|| {
            for _ in 0..10000000 {
                let do_alloc: bool = random();
                if do_alloc {
                    let size = rng.gen_range(1..(SIZE_BITS - 6));
                    let mem = allocator.alloc(size);

                    if let Some(m) = mem {
                        allocs.push((m, size));
                    }
                } else {
                    let len = allocs.len();
                    if len != 0 {
                        let idx = rng.gen_range(0..len);
                        let mem = allocs.remove(idx);

                        allocator.dealloc(mem.0, mem.1)
                    }
                }
            }
        })
    });
}

fn bench_tree(bench: &mut Criterion) {
    use page_allocator::buddy_allocator_tree::BuddyAllocator;

    bench.bench_function("bench_tree", move |bh| {
        const SIZE_BITS: u8 = 21;

        let memory: Vec<u8> = Vec::with_capacity((1 << SIZE_BITS) * BuddyAllocator::PAGE_SIZE);

        let mut allocs: Vec<(usize, u8)> = vec![];
        let mut rng = rand::thread_rng();
        let addr = memory.as_ptr() as usize;

        let mut allocator = BuddyAllocator::new(addr, SIZE_BITS);

        bh.iter(|| {
            for _ in 0..10000000 {
                let do_alloc: bool = random();
                if do_alloc {
                    let size = rng.gen_range(1..(SIZE_BITS - 6));
                    let mem = allocator.alloc(size);

                    if let Some(m) = mem {
                        allocs.push((m, size));
                    }
                } else {
                    let len = allocs.len();
                    if len != 0 {
                        let idx = rng.gen_range(0..len);
                        let mem = allocs.remove(idx);

                        allocator.dealloc(mem.0, mem.1)
                    }
                }
            }
        })
    });
}
