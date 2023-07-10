use criterion::{Criterion, black_box, criterion_group};
use pathing::*;
use oorandom::Rand32;

fn precompute_ds2map(mut ds2map : DS2Map)
{
    ds2map.precompute();
}

fn criterion_benchmark(c: &mut Criterion) {
    let size: isize = 100;

    let mut rand = Rand32::new(123);
    let mut objects = Vec::new();
    for i in (-size/2)..(size/2) {
        for j in (-size/2)..(size/2) {
            if rand.rand_range(1..101) < 5 {
                objects.push((i, j));
            }
        }
    }
    let new_pfg: DS2Map = DS2Map::new().with_objects(objects);
    c.bench_function("precompute_ds2map", |b| b.iter(|| black_box(precompute_ds2map(new_pfg.clone()))));
    // let x = pf.find_path_and_cache(start, end);
    // println!("{:?}", x);
}

criterion_group!(precompute, criterion_benchmark);