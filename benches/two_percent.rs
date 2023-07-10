use criterion::{Criterion, black_box, criterion_group};
use pathing::*;
use oorandom::Rand32;

fn two_ds2map(ds2map : &DS2Map, start : GridPos, end : GridPos)
{
    ds2map.find_path(start, end);
}

fn criterion_benchmark(c: &mut Criterion) {
    let size: isize = 100;
    let mut rand = Rand32::new(123);
    let mut objects = Vec::new();
    for i in (-size/2)..(size/2) {
        for j in (-size/2)..(size/2) {
            if rand.rand_range(1..101) < 10 {
                objects.push((i, j));
            }
        }
    }
    let mut new_pfg: DS2Map = DS2Map::new().with_objects(objects);
    new_pfg.precompute();
    let start = GridNode { x : -((size / 2) as isize - 2), z : -((size / 2) as isize - 2) };
    let end = GridNode { x : (size / 2) as isize - 2, z : (size / 2) as isize - 2 };
    let start = start.into();
    let end = end.into();
    c.bench_function("two_ds2map", |b| b.iter(|| black_box(two_ds2map(&mut new_pfg, start, end))));
}

criterion_group!(two_percent, criterion_benchmark);