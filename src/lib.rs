use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}, fmt::Display};
use pathfinding::prelude::astar;
use vmap::VMap;

pub type GridPos = (isize, isize);
pub type Map<K, V> = HashMap<K, V>;
pub type Set<T> = HashSet<T>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DS2Map {
    blocks : Set<GridPos>,
    objects: VMap<GridPos, GridPos>,
}

impl DS2Map {
    pub fn new() -> Self {
        Self {
            blocks : Set::default(),
            objects: VMap::new(),
        }
    }

    pub fn with_objects(mut self, objects : Vec<GridPos>) -> Self {
        for object in objects {
            self.blocks.insert(object);
        }
        self
    }

    pub fn add_objects(&mut self, objects : Vec<GridPos>) {
        for object in objects {
            self.blocks.insert(object);
        }
    }

    pub fn remove_objects(&mut self, objects : Vec<GridPos>) {
        for object in objects {
            self.blocks.remove(&object);
        }
    }

    pub fn precompute(&mut self) {
        self.objects.clear();
        let mut visited : Set<GridPos> = Set::default();
        for (x, z) in self.blocks.clone() {
            if visited.contains(&(x, z)) { continue; }
            let cells = self.compute_object((x, z));
            let mut nodes = Set::default();
            for (x, z) in &cells {
                let s = self.blocks.contains(&(*x, z - 1));
                let w = self.blocks.contains(&(x - 1, *z));
                let e = self.blocks.contains(&(x + 1, *z));
                let n = self.blocks.contains(&(*x, z + 1));
                let sw = self.blocks.contains(&(x - 1, z - 1));
                let se = self.blocks.contains(&(x + 1, z - 1));
                let nw = self.blocks.contains(&(x - 1, z + 1));
                let ne = self.blocks.contains(&(x + 1, z + 1));
                if !(s || sw || w) {
                    nodes.insert((x - 1, z - 1));
                }
                if !(s || se || e) {
                    nodes.insert((x + 1, z - 1));
                }
                if !(n || nw || w) {
                    nodes.insert((x - 1, z + 1));
                }
                if !(n || ne || e) {
                    nodes.insert((x + 1, z + 1));
                }
            }
            visited.extend(cells.clone().into_iter());
            let object_cells = cells.into_iter().collect();
            let object_nodes = nodes.into_iter().collect();
            self.objects.insert_all(object_cells, object_nodes);
        }
    }

    pub fn precomputed(mut self) -> Self {
        self.precompute();
        self
    }

    pub fn compute_object(&mut self, (x, z): GridPos) -> Set<GridPos> {
        let mut stack : VecDeque<GridPos> = VecDeque::from([
            (x, z)
        ]);
        let mut visited_cells : Set<GridPos> = Set::default();
        while let Some((x, z,)) = stack.pop_front() {
            {
                let (x, z) = (x - 1, z);
                let w = self.blocks.contains(&(x, z));
                if w && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z);
                let e = self.blocks.contains(&(x, z));
                if e && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x, z - 1);
                let s = self.blocks.contains(&(x, z));
                if s && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x, z + 1);
                let n = self.blocks.contains(&(x, z));
                if n && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x - 1, z - 1);
                let sw = self.blocks.contains(&(x, z));
                if sw && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z - 1);
                let se = self.blocks.contains(&(x, z));
                if se && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x - 1, z + 1);
                let nw = self.blocks.contains(&(x, z));
                if nw && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z + 1);
                let ne = self.blocks.contains(&(x, z));
                if ne && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            visited_cells.insert((x, z));
        }
        visited_cells
    }

    pub fn compute_visibility(&self, (x1, z1) : GridPos, (x2, z2) : GridPos) -> Option<GridPos> {

        let mut dx = (x2 - x1).abs();
        let mut dz = (z2 - z1).abs();

        let mut x = x1;
        let mut z = z1;

        let mut n = 0 + dx + dz;

        let x_inc = if x2 > x1 { 1 } else if x2 < x1 { -1 } else { 0 };
        let z_inc = if z2 > z1 { 1 } else if z2 < z1 { -1 } else { 0 };

        let mut error = dx - dz;

        dx *= 2;
        dz *= 2;

        while n > 0 {
            match error.cmp(&0) {
                Ordering::Greater => {
                    if self.is_blocked(x + x_inc, z) {
                        return Some((x + x_inc, z));
                    }
                    x += x_inc;
                    error -= dz;
                    n -= 1;
                }
                Ordering::Less => {
                    if self.is_blocked(x, z + z_inc) {
                        return Some((x, z + z_inc));
                    }
                    z += z_inc;
                    error += dx;
                    n -= 1;
                }
                Ordering::Equal => {
                    match (self.is_blocked(x + x_inc, z) && self.is_blocked(x, z + z_inc), self.is_blocked(x + x_inc, z + z_inc)) {
                        (true, _) => {
                            return Some((x, z + z_inc));
                        },
                        (false, true) => {
                            return Some((x + x_inc, z + z_inc));
                        },
                        _ => {
                            x += x_inc;
                            z += z_inc;
                            error -= dz;
                            error += dx;
                            n -= 2;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn closest_unblocked_cell(&self, (x, z): GridPos) -> GridPos {
        match self.objects.get_value(&(x, z)) {
            Some(nodes) => {
                let mut nodes = VecDeque::from(nodes.clone());
                let (mut closest_node, mut closest_distance) = ((x, z), f32::MAX);
                while let Some(n) = nodes.pop_front() {
                    let distance = distance((x, z), n) as f32;
                    if distance < closest_distance {
                        closest_node = n;
                        closest_distance = distance;
                    }
                }
                closest_node
            },
            None => {
                (x, z)
            }
        }
    }

    pub fn blocks(&self) -> &Set<GridPos> {
        &self.blocks
    }

    pub fn is_blocked(&self, x : isize, y : isize) -> bool {
        self.blocks.contains(&(x, y))
    }

    pub fn object_nodes(&self, pos: GridPos) -> Option<&Vec<GridPos>> {
        self.objects.get_value(&pos)
    }

    pub fn is_node(&self, x : isize, y : isize) -> bool {
        for object in self.objects.values().iter() {
            if object.contains(&(x, y).into()) {
                return true;
            }
        }
        return false;
    }

    pub fn bounds(&self) -> (isize, isize, isize, isize) {
        self.blocks().iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (
                    min_x.min(*x),
                    max_x.max(*x),
                    min_y.min(*y),
                    max_y.max(*y)
                )
            }
        )
    }

    pub fn get_visible_object_nodes(&self, node : GridPos, cell : GridPos) -> Vec<(GridPos, usize)> {
        let mut visited_objects : Set<usize> = Set::default();
        let mut visible_nodes : Vec<(GridPos, usize)> = Vec::new();
        let mut nodes : VecDeque<GridPos> = VecDeque::from(self.objects.get_value(&cell).unwrap().clone());

        while let Some(n) = nodes.pop_front() {
            match self.compute_visibility(node, n) {
                Some(c) => {
                    let current_index = self.objects.get_index(&c).unwrap();

                    if !visited_objects.contains(current_index) && self.objects.get_index(&cell).unwrap() != current_index {
                        visited_objects.insert(*current_index);
                        if let Some(new_nodes) = self.objects.get_value(&c) {
                            nodes.extend(new_nodes);
                        }
                    }
                },
                None => {
                    visible_nodes.push((n, distance(node, n)));
                }
            }
        }

        visible_nodes
    }

    pub fn find_path(&self, start : GridPos, end : GridPos) -> Option<Vec<GridPos>> {
        let start = self.closest_unblocked_cell(start);
        let end = self.closest_unblocked_cell(end);
        astar(&start,
            |node| {
                self.compute_visibility(*node, end).map_or_else(
                    || vec![(end, distance(*node, end))],
                    |c| self.get_visible_object_nodes(*node, c)
                )
            },
            |node| {
                distance(*node, end)
            },
            |node| *node == end
        )
        .map(|(mut path, _)| { self.prune(&mut path); path } )
    }

    pub fn prune(&self, path: &mut Vec<GridPos>) {
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
pub fn distance((x1, z1) : GridPos, (x2, z2) : GridPos) -> usize {
    (((x2 * 10 - x1 * 10).pow(2) + (z2 * 10 - z1 * 10).pow(2)) as f32) as usize
}

impl Display for DS2Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        let capacity = (bounds.2..=bounds.3).count() * ((bounds.0..=bounds.1).count() * 5 + 3) + (bounds.0..=bounds.1).count() * 3 * 3 + (bounds.0..=bounds.1).count() * 3 + 3;
        let mut result = String::with_capacity(capacity);
        result.push_str(" ");
        for _ in bounds.0..=bounds.1 {
            result.push_str("___");
        }
        result.push_str("\n");
        for y in bounds.2..=bounds.3 {
            let mut slice = String::new();
            slice.push_str("|");
            for x in bounds.0..=bounds.1 {
                if self.is_node(x, y) {
                    slice.push_str("[\u{25A0}]");
                } else if self.is_blocked(x, y) {
                    slice.push_str("[\u{25A1}]");
                } else {
                    slice.push_str("   ");
                }
            }
            result.push_str(&slice);
            result.push_str("|\n");
        }
        result.push_str(" ");
        for _ in bounds.0..=bounds.1 {
            result.push_str("\u{203E}\u{203E}\u{203E}");
        }
        write!(f, "{}", result)
    }
}