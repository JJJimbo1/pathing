pub mod d2;
pub mod ds2;

pub type ObjectId = usize;
pub type GridPos = (isize, isize);

impl From<GridCell> for GridPos {
    fn from(value: GridCell) -> Self {
        (value.x, value.z)
    }
}

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

impl From<GridCell> for GridNode {
    fn from(cell : GridCell) -> Self {
        Self {
            x : cell.x,
            z : cell.z,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GridCell {
    pub x : isize,
    pub z : isize,
    pub blocked : bool,
}

impl GridCell {
    pub fn new(x : isize, z : isize, blocked : bool) -> Self {
        Self {
            x,
            z,
            blocked,
        }
    }

    pub fn index(&self) -> (isize, isize) {
        (self. x, self.z)
    }
}

impl From<GridPos> for GridCell {
    fn from(cell : GridPos) -> Self {
        Self {
            x : cell.0,
            z : cell.1,
            blocked: false,
        }
    }
}

impl From<GridNode> for GridCell {
    fn from(node : GridNode) -> Self {
        Self {
            x : node.x,
            z : node.z,
            blocked : false,
        }
    }
}