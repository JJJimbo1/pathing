use criterion::{Criterion, black_box, criterion_group};
use pathing::*;
use pathing::d2::*;
use pathing::ds2::*;

fn empty_d2map(d2map : &D2Map, start : GridCell, end : GridCell)
{
    d2map.find_path(start, end);
}

fn empty_ds2map(ds2map : &DS2Map, start : GridPos, end : GridPos)
{
    ds2map.find_path(start, end);
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 100;

    let mut old_pfg = D2Map::new(size, size).with_cells(|x, z| GridCell { x, z, blocked : false});
    old_pfg.precompute();
    let start = GridCell::from(GridNode { x : -((size / 2) as isize - 2), z : -((size / 2) as isize - 2) });
    let end = GridCell::from(GridNode { x : (size / 2) as isize - 2, z : (size / 2) as isize - 2 });
    c.bench_function("empty_d2map", |b| b.iter(|| black_box(empty_d2map(&mut old_pfg, start, end))));

    let mut new_pfg: DS2Map = DS2Map::new();
    new_pfg.precompute();
    let start = start.into();
    let end = end.into();
    c.bench_function("empty_ds2map", |b| b.iter(|| black_box(empty_ds2map(&mut new_pfg, start, end))));
}

criterion_group!(empty_map, criterion_benchmark);