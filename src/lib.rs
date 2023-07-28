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
    blocks : Set<GridPos>,
    objects: VMap<GridPos, GridNode>,
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

    pub fn precompute(&mut self) {
        self.objects.clear();
        let mut visited : Set<GridPos> = Set::default();
        for (x, z) in self.blocks.clone() {
            if visited.contains(&(x, z)) { continue; }
            let cells = self.compute_object((x, z));
            let mut nodes = Set::default();
            for (x, z) in cells.clone() {
                let s = self.blocks.contains(&(x, z - 1));
                let w = self.blocks.contains(&(x - 1, z));
                let e = self.blocks.contains(&(x + 1, z));
                let n = self.blocks.contains(&(x, z + 1));
                let sw = self.blocks.contains(&(x - 1, z - 1));
                let se = self.blocks.contains(&(x + 1, z - 1));
                let nw = self.blocks.contains(&(x - 1, z + 1));
                let ne = self.blocks.contains(&(x + 1, z + 1));
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
                if self.is_blocked(x + x_inc, z) {
                    return Some((x + x_inc, z));
                }
                x += x_inc;
                error -= dz;
                n -= 1;
            } else if error < 0 {
                if self.is_blocked(x, z + z_inc) {
                    return Some((x, z + z_inc));
                }
                z += z_inc;
                error += dx;
                n -= 1;
            } else {
                if self.is_blocked(x + x_inc, z)
                && self.is_blocked(x, z + z_inc) {
                    return Some((x, z + z_inc));
                }
                if self.is_blocked(x + x_inc, z + z_inc) {
                    return Some((x + x_inc, z + z_inc));
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

    pub fn blocks(&self) -> &Set<GridPos> {
        &self.blocks
    }

    pub fn is_blocked(&self, x : isize, y : isize) -> bool {
        self.blocks.contains(&(x, y))
    }

    pub fn object_nodes(&self, pos: GridPos) -> Option<&Vec<GridNode>> {
        self.objects.get_value(&pos)
    }

    pub fn is_node(&self, x : isize, y : isize) -> bool {
        self.objects.values().clone().into_iter().flatten().collect::<Vec<GridNode>>().contains(&(x, y).into())
    }

    pub fn bounds(&self) -> (isize, isize, isize, isize) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for block in self.blocks() {
            if block.0 < min_x { min_x = block.0; }
            if block.0 > max_x { max_x = block.0; }
            if block.1 < min_y { min_y = block.1; }
            if block.1 > max_y { max_y = block.1; }
        }
        (min_x, max_x, min_y, max_y)
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
        println!("{:?}", start);
        let Some(end) = self.closest_unblocked_cell(end).and_then(|e| Some(e.into())) else { return None; };
        println!("{:?}", end);
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


pub fn display_with_path(grid: &DS2Map, path: Vec<GridNode>) {
    println!("{:?}", grid.bounds());
    let bounds = grid.bounds();
    let mut result = String::new();
    for y in bounds.2..=bounds.3 {
        let mut slice = String::new();
        for x in bounds.0..=bounds.1 {
            if path.contains(&(x, y).into()) {
                slice.push_str("[T]");
            } else if grid.is_node(x, y) {
                slice.push_str("[-]");
            } else if grid.is_blocked(x, y) {
                slice.push_str("[+]");
            } else {
                slice.push_str("[ ]");
            }
        }
        result.push_str(&slice);
        result.push_str("\n");
    }
    println!("{}", result);
}

#[test]
fn atest() {
    use oorandom::Rand32;
    let size = 40;
    let mut rand = Rand32::new(123);
    let mut objects = Vec::new();
    for i in (-size/2)..=(size/2) {
        for j in (-size/2)..=(size/2) {
            // if i > -1 && i < 1 && j > -1 && j < 1 || i == -20 || j == 20 {
            //     objects.push((i, j));
            // }
            if rand.rand_range(1..101) < 10 {
                objects.push((i, j));
            }
        }
    }
    let mut grid: DS2Map = DS2Map::new().with_objects(objects);
    grid.precompute();
    let start = GridNode { x : -((size / 2) as isize - 2), z : -((size / 2) as isize - 2) };
    let end = GridNode { x : (size / 2) as isize - 2, z : (size / 2) as isize - 2 };
    // println!("{:?}", grid.compute_visibility(start, end));
    let start = start.into();
    let end = end.into();
    let path = grid.find_path(start, end);
    display_with_path(&grid, path.unwrap());
}