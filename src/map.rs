use lore_render::ObjectInstance;

pub const TILE_SIZE: f32 = 30.0;

pub struct Map {
    tiles: [[Option<Tile<'static>>;64];64],
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: [[None; 64]; 64],
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile<'a> {
    prefab: &'a TilePrefab,
    encounters: u8, // TODO placeholder type
}

struct TilePrefab {
    objects: Vec<(usize, ObjectInstance)>,
    exits: u8, // bitmask on Direction
}

impl TilePrefab {
    pub fn has_exit(&self, dir: Direction) -> bool {
        self.exits & (dir as u8) != 0
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    NORTH = 1,
    SOUTH = 2,
    EAST = 4,
    WEST = 8,
}

impl Direction {
    pub fn counterclockwise(self) -> Direction {
        match self {
            Self::NORTH => Self::WEST,
            Self::WEST => Self::SOUTH,
            Self::SOUTH => Self::EAST,
            Self::EAST => Self::NORTH,
        }
    }

    pub fn clockwise(self) -> Direction {
        match self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH,
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Self::NORTH => Self::SOUTH,
            Self::EAST => Self::WEST,
            Self::SOUTH => Self::NORTH,
            Self::WEST => Self::EAST,
        }
    }
}