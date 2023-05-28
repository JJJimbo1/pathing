use std::collections::VecDeque;
use centered_grid::Grid2;
use indexmap::{IndexMap, IndexSet};
use hashbrown::hash_map::DefaultHashBuilder;
use pathfinding::prelude::astar;
use crate::*;

pub type AIndexMap<K, V> = IndexMap<K, V, DefaultHashBuilder>;
pub type AIndexSet<T> = IndexSet<T, DefaultHashBuilder>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct D2Map {
    grid : Grid2<GridCell>,
    cell_objects : AIndexMap<GridCell, ObjectId>,
    object_nodes : AIndexMap<ObjectId, VecDeque<GridNode>>,
    searched_nodes : AIndexMap<(GridNode, ObjectId), Vec<(GridNode, usize)>>,
    object_id_counter : usize,
}

impl D2Map {
    pub fn new(width : usize, length : usize) -> Self {
        Self {
            grid : Grid2::new(width, length, GridCell::default()),
            object_nodes : AIndexMap::default(),
            cell_objects : AIndexMap::default(),
            searched_nodes : AIndexMap::default(),
            object_id_counter : 0,
        }
    }

    pub fn with_cells<FC>(mut self, mut cell : FC) -> Self
        where FC : FnMut(isize, isize) -> GridCell
    {
        for i in self.grid.range_x() {
            for j in self.grid.range_y() {
                self.grid[(i, j)] = cell(i, j);
            }
        }
        self
    }

    pub fn precompute(&mut self) {

        self.cell_objects.clear();
        self.object_nodes.clear();
        self.searched_nodes.clear();
        self.object_id_counter = 0;

        let mut visited : AIndexSet<GridCell> = IndexSet::default();
        let mut object_nodes : AIndexMap<ObjectId, AIndexSet<GridNode>> = AIndexMap::default();

        for i in self.grid.range_x() {
            for j in self.grid.range_y() {

                let cell = self.grid[(i, j)];
                if cell.blocked && !visited.contains(&cell) {
                    let cells = self.compute_object(cell);
                    object_nodes.insert(cells.1, AIndexSet::default());

                    for c in &cells.0 {
                        if c.x > self.grid.lens_x().0 && c.z > self.grid.lens_y().0 {
                            let s = self.grid[(c.x, c.z - 1)];
                            let sw = self.grid[(c.x - 1, c.z - 1)];
                            let w = self.grid[(c.x - 1, c.z)];
                            if !(s.blocked || sw.blocked || w.blocked) {
                                object_nodes.get_mut(&cells.1).unwrap().insert(GridNode::from(sw));
                            }
                        }
                        if c.x < self.grid.lens_y().1 - 1 && c.z > self.grid.lens_y().0 {
                            let s = self.grid[(c.x, c.z - 1)];
                            let se = self.grid[(c.x + 1, c.z - 1)];
                            let e = self.grid[(c.x + 1, c.z)];
                            if !(s.blocked || se.blocked || e.blocked) {
                                object_nodes.get_mut(&cells.1).unwrap().insert(GridNode::from(se));
                            }
                        }
                        if c.x > self.grid.lens_x().0 && c.z < self.grid.lens_y().1 - 1 {
                            let n = self.grid[(c.x, c.z + 1)];
                            let nw = self.grid[(c.x - 1, c.z + 1)];
                            let w = self.grid[(c.x - 1, c.z)];
                            if !(n.blocked || nw.blocked || w.blocked) {
                                object_nodes.get_mut(&cells.1).unwrap().insert(GridNode::from(nw));
                            }
                        }
                        if c.x < self.grid.lens_x().1 - 1 && c.z < self.grid.lens_y().1 - 1 {
                            let n = self.grid[(c.x, c.z + 1)];
                            let ne = self.grid[(c.x + 1, c.z + 1)];
                            let e = self.grid[(c.x + 1, c.z)];
                            if !(n.blocked || ne.blocked || e.blocked) {
                                object_nodes.get_mut(&cells.1).unwrap().insert(GridNode::from(ne));
                            }
                        }
                    }

                    let object = cells.0.iter().map(|i| (*i, cells.1)).collect::<AIndexMap<GridCell, usize>>();
                    self.cell_objects.extend(&object);
                    visited.extend(cells.0);
                }
            }
        }
        self.object_nodes = object_nodes
            .into_iter()
            .map(|(id, nodes)| {
                (id, nodes
                    .into_iter()
                    .collect())
            })
            .collect();
    }

    pub fn precomputed(mut self) -> Self {
        self.precompute();
        self
    }

    pub fn compute_object(&mut self, cell : GridCell) -> (AIndexSet<GridCell>, usize) {
        let mut stack : VecDeque<GridCell> = VecDeque::from([cell]);
        let mut visited_cells : AIndexSet<GridCell> = AIndexSet::default();
        while let Some(c) = stack.pop_front() {
            if c.x > self.grid.lens_x().0 {
                let w = self.grid[(c.x - 1, c.z)];
                if w.blocked && !visited_cells.contains(&w) && !stack.contains(&w) { stack.push_back(w); }
            }
            if c.x < self.grid.lens_x().1 - 1 {
                let e = self.grid[(c.x + 1, c.z)];
                if e.blocked && !visited_cells.contains(&e) && !stack.contains(&e) { stack.push_back(e); }
            }
            if c.z > self.grid.lens_y().0 {
                let s = self.grid[(c.x, c.z - 1)];
                if s.blocked && !visited_cells.contains(&s) && !stack.contains(&s) { stack.push_back(s); }
            }
            if c.z < self.grid.lens_y().1 - 1 {
                let n = self.grid[(c.x, c.z + 1)];
                if n.blocked && !visited_cells.contains(&n) && !stack.contains(&n) { stack.push_back(n); }
            }
            if c.x > self.grid.lens_x().0 && c.z > self.grid.lens_y().0 {
                let sw = self.grid[(c.x - 1, c.z - 1)];
                if sw.blocked && !visited_cells.contains(&sw) && !stack.contains(&sw) { stack.push_back(sw); }
            }
            if c.x < self.grid.lens_x().1 - 1 && c.z > self.grid.lens_y().0 {
                let se = self.grid[(c.x + 1, c.z - 1)];
                if se.blocked && !visited_cells.contains(&se) && !stack.contains(&se) { stack.push_back(se); }
            }
            if c.x > self.grid.lens_x().0 && c.z < self.grid.lens_y().1 - 1 {
                let nw = self.grid[(c.x - 1, c.z + 1)];
                if nw.blocked && !visited_cells.contains(&nw) && !stack.contains(&nw) { stack.push_back(nw); }
            }
            if c.x < self.grid.lens_x().1 - 1 && c.z < self.grid.lens_y().1 - 1 {
                let ne = self.grid[(c.x + 1, c.z + 1)];
                if ne.blocked && !visited_cells.contains(&ne) && !stack.contains(&ne) { stack.push_back(ne); }
            }
            visited_cells.insert(c);
        }
        (visited_cells, self.next_object_id())
    }

    pub fn next_object_id(&mut self) -> usize {
        self.object_id_counter += 1;
        self.object_id_counter - 1
    }

    pub fn compute_visibility(&self, start : GridNode, end : GridNode) -> Option<GridCell> {

        let object = self.cell_objects.get(&GridCell::from(start));

        let x0 = start.x;
        let z0 = start.z;
        let x1 = end.x;
        let z1 = end.z;

        let mut dx = (x1 - x0).abs();
        let mut dz = (z1 - z0).abs();

        let mut x = x0;
        let mut z = z0;

        let mut n = 0 + dx + dz;

        let x_inc = if x1 > x0 { 1 } else if x1 < x0 { -1 } else { 0 };
        let z_inc = if z1 > z0 { 1 } else if z1 < z0 { -1 } else { 0 };

        let mut error = dx - dz;

        dx *= 2;
        dz *= 2;

        while n > 0 {
            if error > 0 {
                if self.get_cell(x + x_inc, z).map_or(false, |c| c.blocked)
                && object.map_or(true, |id| *id != self.cell_objects[&self.grid[(x + x_inc, z)]]) {
                    return Some(self.grid[(x + x_inc, z)]);
                }
                x += x_inc;
                error -= dz;
                n -= 1;
            } else if error < 0 {
                if self.get_cell(x, z + z_inc).map_or(false, |c| c.blocked)
                && object.map_or(true, |id| *id != self.cell_objects[&self.grid[(x, z + z_inc)]]) {
                    return Some(self.grid[(x, z + z_inc)]);
                }
                z += z_inc;
                error += dx;
                n -= 1;
            } else {
                if self.get_cell(x + x_inc, z).map_or(false, |c| c.blocked)
                && self.get_cell(x, z + z_inc).map_or(false, |c| c.blocked)
                && object.map_or(true, |id| *id != self.cell_objects[&self.grid[(x, z + z_inc)]]) {
                    return Some(self.grid[(x, z + z_inc)]);
                }
                x += x_inc;
                z += z_inc;
                error -= dz;
                error += dx;
                n -= 2;
            }
        }
        None
    }

    pub fn closest_unblocked_cell(&self, cell: GridCell) -> Option<GridCell> {
        match self.cell_objects.get(&cell) {
            Some(x) => {
                let mut nodes = self.object_nodes[*x].clone();
                let (mut closest_node, mut closest_distance) = (None, f32::MAX);
                while let Some(n) = nodes.pop_front() {
                    if (distance(cell.into(), n) as f32) < closest_distance {
                        closest_node = Some(n.into());
                        closest_distance = distance(cell.into(), n) as f32;
                    }
                }
                closest_node
            },
            None => {
                Some(cell)
            }
        }
    }

    pub fn grid(&self) -> &Grid2<GridCell> {
        &self.grid
    }

    pub fn get_cell(&self, x : isize, y : isize) -> Option<&GridCell> {
        self.grid.get((x, y))
    }

    pub fn get_cell_mut(&mut self, x : isize, y : isize) -> Option<&mut GridCell> {
        self.grid.get_mut((x, y))
    }

    pub fn get_visible_cell_object_nodes(&self, node : GridNode, cell : GridCell) -> Vec<(GridNode, usize)> {
        let mut visited_objects : AIndexSet<usize> = AIndexSet::default();
        let mut visible_nodes : Vec<(GridNode, usize)> = Vec::new();
        let mut nodes : VecDeque<GridNode> = self.object_nodes[&self.cell_objects[&cell]].clone();

        while let Some(n) = nodes.pop_front() {
            match self.compute_visibility(node, n) {
                Some(c) => {
                    if !visited_objects.contains(&self.cell_objects[&c]) && self.cell_objects[&cell] != self.cell_objects[&c] {
                        visited_objects.insert(self.cell_objects[&c]);
                        nodes.append(&mut self.object_nodes[&self.cell_objects[&c]].clone());
                    }
                },
                None => {
                    visible_nodes.push((n, distance(node, n)));
                }
            }
        }

        visible_nodes
    }

    pub fn cache_visible_cell_object_nodes(&mut self, node : GridNode, cell : GridCell, nodes : Vec<(GridNode, usize)>) {
        let obj = self.cell_objects[&cell];
        self.searched_nodes.insert((node, obj), nodes);
    }

    pub fn find_path(&self, start : GridCell, end : GridCell) -> Option<Vec<GridNode>> {
        let Some(start) = self.closest_unblocked_cell(start).and_then(|s| Some(s.into())) else { return None; };
        let Some(end) = self.closest_unblocked_cell(end).and_then(|e| Some(e.into())) else { return None; };
        astar(&start,
            |node| {
                self.compute_visibility(*node, end).map_or_else(
                    || vec![(end, distance(*node, end))],
                    |c| self.get_visible_cell_object_nodes(*node, c)
                )
            },
            |node| {
                distance(*node, end)
            },
            |node| *node == end
        )
        .map(|mut f| { self.prune(&mut f.0); f.0 } )
    }

    pub fn find_path_and_cache(&mut self, start : GridCell, end : GridCell) -> Option<Vec<GridNode>> {
        let Some(start) = self.closest_unblocked_cell(start).and_then(|s| Some(s.into())) else { return None; };
        let Some(end) = self.closest_unblocked_cell(end).and_then(|e| Some(e.into())) else { return None; };
        astar(&start,
            |node| {
                if let Some(c) = self.compute_visibility(*node, end) {
                    let obj = self.cell_objects[&c];
                    self.searched_nodes.get(&(*node, obj))
                        .cloned()
                        .unwrap_or_else(|| {
                            let nodes = self.get_visible_cell_object_nodes(*node, c);
                            self.cache_visible_cell_object_nodes(*node, c, nodes);
                            self.searched_nodes[&(*node, obj)].clone()
                        }
                    )
                } else {
                    vec![(end, distance(*node, end))]
                }
            },
            |node| {
                distance(*node, end)
            },
            |node| *node == end
        )
        .map(|mut f| { self.prune(&mut f.0); f.0 } )
    }

    pub fn prune(&self, path : &mut Vec<GridNode>) {
        let mut n = 0;
        while n + 2 < path.len() {
            if self.compute_visibility(path[n], path[n + 2]).is_none() {
                path.remove(n + 1);
            } else {
                n += 1;
            }
        }
    }
}

#[inline]
pub fn distance(n1 : GridNode, n2 : GridNode) -> usize {
    (((n2.x * 10 - n1.x * 10).pow(2) + (n2.z * 10 - n1.z * 10).pow(2)) as f32).sqrt() as usize

}
