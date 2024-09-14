use criterion::{Criterion, black_box, criterion_group};
use pathing::*;

fn empty_ds2map(ds2map : &DS2Map, start : GridPos, end : GridPos)
{
    ds2map.find_path(start, end);
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 100;
    let objects = Vec::new();
    let mut new_pfg: DS2Map = DS2Map::new().with_objects(objects);
    new_pfg.precompute();
    let start = ( -((size / 2) as isize - 2), -((size / 2) as isize - 2) );
    let end = ((size / 2) as isize - 2, (size / 2) as isize - 2 );
    c.bench_function("empty_ds2map", |b| b.iter(|| black_box(empty_ds2map(&mut new_pfg, start, end))));
}

criterion_group!(empty_map, criterion_benchmark);