const CHUNK_BIT_WIDTH: usize = 6;
const CHUNK_BIT_HEIGHT: usize = 6;

const CHUNK_WIDTH: usize  = 1 << CHUNK_BIT_WIDTH;
const CHUNK_HEIGHT: usize = 1 << CHUNK_BIT_HEIGHT;

const BITMASK: usize = 0x3F;

pub struct Terrain {
    chunks: [Chunk; 1]
}

impl Terrain {
    pub fn new() -> Self {
        Terrain {
            chunks: [Chunk::new()]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let chunk_x = x >> CHUNK_BIT_WIDTH;

        let x = x & BITMASK;

        self.chunks[chunk_x].set(x, y, value);  
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let chunk_x = x >> CHUNK_BIT_WIDTH;

        let x = x & BITMASK;

        self.chunks[chunk_x].get(x, y)
    }
}

struct Chunk {
    value: [[bool; CHUNK_HEIGHT]; CHUNK_WIDTH]
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            value: [[false; CHUNK_HEIGHT]; CHUNK_WIDTH]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.value[x][y] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.value[x][y]
    }
}

pub fn terrain_hills(terrain: &mut Terrain) {
    for x in 0..CHUNK_WIDTH {
        for y in 0..x {
            terrain.set(x, y, true);
        }
    }
}