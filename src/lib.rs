use std::collections::VecDeque;
use pathfinding::prelude::astar;
use fxhash::*;
use valley_map::VMap;

pub type ObjectId = usize;
pub type GridPos = (isize, isize);
pub type Map<K, V> = FxHashMap<K, V>;
pub type Set<T> = FxHashSet<T>;

impl From<GridNode> for GridPos {
    fn from(value: GridNode) -> Self {
        (value.x, value.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridNode {
    pub x : isize,
    pub z : isize,
}

impl GridNode {
    pub fn new(x : isize, z : isize) -> Self {
        Self {
            x,
            z,
        }
    }

    pub fn pos(&self) -> GridPos {
        (self. x, self.z)
    }
}

impl From<GridPos> for GridNode {
    fn from((x, z): GridPos) -> Self {
        Self {
            x,
            z,
        }
    }
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DS2Map {
    blocked : Set<GridPos>,
    objects: VMap<GridPos, GridNode>,
}

impl DS2Map {
    pub fn new() -> Self {
        Self {
            blocked : Set::default(),
            objects: VMap::new(),
        }
    }

    pub fn with_objects(mut self, objects : Vec<GridPos>) -> Self {
        for object in objects {
            self.blocked.insert(object);
        }
        self
    }

    pub fn precompute(&mut self) {
        self.objects.clear();
        let mut visited : Set<GridPos> = Set::default();
        for (x, z) in self.blocked.clone() {
            if visited.contains(&(x, z)) { continue; }
            let cells = self.compute_object((x, z));
            let mut nodes = Set::default();
            for (x, z) in cells.clone() {
                let s = self.blocked.contains(&(x, z - 1));
                let w = self.blocked.contains(&(x - 1, z));
                let e = self.blocked.contains(&(x + 1, z));
                let n = self.blocked.contains(&(x, z + 1));
                let sw = self.blocked.contains(&(x - 1, z - 1));
                let se = self.blocked.contains(&(x + 1, z - 1));
                let nw = self.blocked.contains(&(x - 1, z + 1));
                let ne = self.blocked.contains(&(x + 1, z + 1));
                if !(s || sw || w) {
                    nodes.insert(GridNode::from((x - 1, z - 1)));
                }
                if !(s || se || e) {
                    nodes.insert(GridNode::from((x + 1, z - 1)));
                }
                if !(n || nw || w) {
                    nodes.insert(GridNode::from((x - 1, z + 1)));
                }
                if !(n || ne || e) {
                    nodes.insert(GridNode::from((x + 1, z + 1)));
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
                let w = self.blocked.contains(&(x, z));
                if w && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z);
                let e = self.blocked.contains(&(x, z));
                if e && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x, z - 1);
                let s = self.blocked.contains(&(x, z));
                if s && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x, z + 1);
                let n = self.blocked.contains(&(x, z));
                if n && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x - 1, z - 1);
                let sw = self.blocked.contains(&(x, z));
                if sw && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z - 1);
                let se = self.blocked.contains(&(x, z));
                if se && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x - 1, z + 1);
                let nw = self.blocked.contains(&(x, z));
                if nw && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            {
                let (x, z) = (x + 1, z + 1);
                let ne = self.blocked.contains(&(x, z));
                if ne && !visited_cells.contains(&(x, z)) && !stack.contains(&(x, z)) { stack.push_back((x, z)); }
            }
            visited_cells.insert((x, z));
        }
        visited_cells
    }

    pub fn compute_visibility(&self, start : GridNode, end : GridNode) -> Option<GridPos> {

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
                if self.is_blocked(x + x_inc, z)
                {
                    return Some((x + x_inc, z));
                }
                x += x_inc;
                error -= dz;
                n -= 1;
            } else if error < 0 {
                if self.is_blocked(x, z + z_inc)
                {
                    return Some((x, z + z_inc));
                }
                z += z_inc;
                error += dx;
                n -= 1;
            } else {
                if self.is_blocked(x + x_inc, z)
                && self.is_blocked(x, z + z_inc)
                {
                    return Some((x, z + z_inc));
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

    pub fn closest_unblocked_cell(&self, (x, z): GridPos) -> Option<GridPos> {
        match self.objects.get_value(&(x, z)) {
            Some(nodes) => {
                let mut nodes = VecDeque::from(nodes.clone());
                let (mut closest_node, mut closest_distance) = (None, f32::MAX);
                while let Some(n) = nodes.pop_front() {
                    if (distance((x, z).into(), n) as f32) < closest_distance {
                        closest_node = Some(n.pos());
                        closest_distance = distance((x, z).into(), n) as f32;
                    }
                }
                closest_node
            },
            None => {
                Some((x, z))
            }
        }
    }

    pub fn objects(&self) -> &Set<GridPos> {
        &self.blocked
    }

    pub fn is_blocked(&self, x : isize, y : isize) -> bool {
        self.blocked.contains(&(x, y))
    }


    pub fn get_visible_cell_object_nodes(&self, node : GridNode, cell : GridPos) -> Vec<(GridNode, usize)> {
        let mut visited_objects : Set<usize> = Set::default();
        let mut visible_nodes : Vec<(GridNode, usize)> = Vec::new();
        let mut nodes : VecDeque<GridNode> = VecDeque::from(self.objects.get_value(&cell).unwrap().clone());

        while let Some(n) = nodes.pop_front() {
            match self.compute_visibility(node, n) {
                Some(c) => {
                    if !visited_objects.contains(self.objects.get_index(&c).unwrap()) && self.objects.get_index(&cell).unwrap() != self.objects.get_index(&c).unwrap() {
                        visited_objects.insert(*self.objects.get_index(&c).unwrap());
                        nodes.append(&mut VecDeque::from(self.objects.get_value(&c).unwrap().clone()));
                    }
                },
                None => {
                    visible_nodes.push((n, distance(node, n)));
                }
            }
        }

        visible_nodes
    }

    pub fn find_path(&self, start : GridPos, end : GridPos) -> Option<Vec<GridNode>> {
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
