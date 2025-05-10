use core::panic;

use super::{Board, I8_R};

pub const X_EDGE: i8 = 0;
pub const Y_EDGE: i8 = 1;
pub const Z_EDGE: i8 = 2;

impl Board {
    
    /**
     * Get a list of the valid tile coordinates
     * 
     * TODO - this can be computed once for efficiency, it will never change for boards of radius R
     */
    pub fn get_tile_coords() -> Vec<Coordinate2> {
        let mut vec = Vec::new();
        for y in -I8_R ..= I8_R {
            for x in -I8_R ..= I8_R {
                if (-x -y).abs() <= I8_R {
                    vec.push(Coordinate2::new(x, y));
                }
            }
        }
        vec
    }

    /**
     * Get a list of the valid vertex coordinates
     * 
     * TODO - this can be computed once for efficiency, it will never change for boards of radius R
     */
    pub fn get_vertex_coords() -> Vec<Coordinate3> {
        let mut vec: Vec<Coordinate3> = Vec::new();
        for x in -I8_R - 1 ..= I8_R + 1 {
            for y in -I8_R - 1 ..= I8_R + 1 {
                for z in -I8_R - 1 ..= I8_R + 1 {
                    let coord = Coordinate3::new(x, y, z);
                    if coord.sum().abs() == 1 && coord.abs_sum() <= I8_R * 2 + 1 {
                        vec.push(coord);
                    }
                }
            }
        }
        vec
    }

    /**
     * Get a list of the valid edge coordinates
     */
    pub fn get_edge_coords() -> Vec<Coordinate3> {
        let mut vec: Vec<Coordinate3> = Vec::new();
        for x in -I8_R - 1 ..= I8_R {
            for y in (-I8_R ..= I8_R + 1).rev() {
                if Coordinate2::new(x, y).is_valid_tile() || Coordinate2::new(x + 1, y + 1).is_valid_tile() {
                    for d in 0 ..= 2 {
                        vec.push(Coordinate3::new(d, x, y));
                    }
                }
            }
        }
        vec
    }

    /**
     * Get the edge coordinate between 2 vertices.
     */
    pub fn get_edge_between_vertices_coords(a: &Coordinate3, b: &Coordinate3) -> Coordinate3 {
        let direction = if a.x == b.x {
            X_EDGE
        } else if a.y == b.y {
            Y_EDGE
        } else if a.z == b.z {
            Z_EDGE
        } else {
            panic!("No edge between {} and {}", a, b);
        };

        // Determine which point is prime and which is not
        let (p, pp) = if a.sum() < 0 {
            (b, a)
        } else {
            (a, b)
        };

        let (x, y) = match direction {
            X_EDGE => (pp.y, p.z),
            Y_EDGE => (pp.z, p.x),
            Z_EDGE => (pp.x, p.y),
            _ => unreachable!()
        };

        Coordinate3::new(direction, x, y)
    }

    /**
     * Returns the associated pair prime vertex for a specified vertex and direction.
     * 
     * Note: if a prime vertex is passed, the regular vertex pair is returned
     */
    pub fn get_prime_pair_vertex(direction: i8, coord: &Coordinate3) -> Coordinate3 {
        let factor = if coord.sum() < 0 { -1 } else { 1 };
        match direction {
            X_EDGE => Coordinate3::new(coord.x, coord.y - factor, coord.z - factor),
            Y_EDGE => Coordinate3::new(coord.x - factor, coord.y , coord.z - factor),
            Z_EDGE => Coordinate3::new(coord.x - factor, coord.y - factor, coord.z),
            _ => unreachable!()
        }
    }

    /**
     * Providing a vertex coordinate, get all the surrounding edge coordinates
     */
    pub fn get_vertex_adjacent_edge_coords(coord: &Coordinate3) -> Vec<Coordinate3> {
        let mut vec =  Vec::new();
        for direction in 0 .. 3 {
            let pair = Self::get_prime_pair_vertex(direction, coord);
            if pair.abs_sum() <= 2 * I8_R + 1 {
                let result: Coordinate3 = Self::get_edge_between_vertices_coords(coord, &pair);
                vec.push(result);
            }
        }
        vec
    }

    /**
     * Get the tiles coordinates adjacent to a vertex coordinate
     */
    pub fn get_vertex_adjacent_tile_coords(coord: &Coordinate3) -> Vec<Coordinate2> {
        if coord.sum() == -1 {
            vec![
                Coordinate2::new(coord.x, coord.y),
                Coordinate2::new(coord.x, coord.y + 1),
                Coordinate2::new(coord.x + 1, coord.y),
            ] 
        } else if coord.sum() == 1 {
            vec![
                Coordinate2::new(coord.x, coord.y - 1),
                Coordinate2::new(coord.x - 1, coord.y),
                Coordinate2::new(coord.x, coord.y),
            ] 
        } else {
            panic!("Invalid vertex coordinate provided");
        }
    }

    /**
     * Get the 6 vertices adjacent to a tile
     */
    pub fn get_tile_adjacent_vertex_coords(coord: &Coordinate2) -> [Coordinate3; 6] {
        let z = -coord.x - coord.y;
        [
            Coordinate3::new(coord.x - 1, coord.y, z),
            Coordinate3::new(coord.x + 1, coord.y, z),
            Coordinate3::new(coord.x, coord.y - 1, z),
            Coordinate3::new(coord.x, coord.y + 1, z),
            Coordinate3::new(coord.x, coord.y, z - 1),
            Coordinate3::new(coord.x, coord.y, z + 1),
        ]
    }

    
    /**
     * Providing an edge coordinate, get all the surrounding vertices coordinates
     */
    pub fn get_edge_adjacent_vertex_coords(coord: &Coordinate3) -> Vec<Coordinate3> {

        // X(-2, 2) -> (0, -1, 2)
        // x + y + z = 1

        // Get the x' and y value of the edge
        let reg = match coord.x {
            // X edge
            0 => {
                let y = coord.y + 1;
                let z = coord.z;
                let x = 1 - y - z;
                Coordinate3::new(x, y, z)
            },
            // Y edge
            1 => {
                let z = coord.y + 1;
                let x = coord.z;
                let y = 1 - x - z;
                Coordinate3::new(x, y, z)
            },
            // Z edge
            2 => {
                let x = coord.y + 1;
                let y = coord.z;
                let z = 1 - x - y;
                Coordinate3::new(x, y, z)
            },
            _ => unreachable!(),
        };


        let prime =  Self::get_prime_pair_vertex(coord.x, &reg);
        vec![reg, prime]

    }

    /**
     * Utility function to get the vertex for radius r
     */
    pub fn get_vertex_count(r: i32) -> i32 {
        if r < 0 {
            0
        } else {
            6 * (1 + 2 * r) + 6 * r + Self::get_vertex_count(r - 1)
        }
    }

    /**
     * Get a list of all the neighbor coordinates to a tile.
     * If a tile is on the edge of the board, those coordinates
     * are not considered.
     */
    pub fn get_tile_neighbor_coords(coord: &Coordinate2) -> Vec<Coordinate2> {
        let mut vec = Vec::new();
        for dx in coord.x - 1 ..= coord.x + 1 {
            for dy in coord.y - 1 ..= coord.y + 1 {
                if dx != coord.x || dy != coord.y {
                    vec.push(Coordinate2::new(dx, dy))
                }
            }
        }

        // The first and last values are never neighbors
        vec.remove(0);
        vec.pop();

        // Filter only valid coords
        vec.into_iter().filter(|e| {
            e.is_valid_tile()
        }).collect()
    }

    /**
     * Get the vertex coordinate between 3 tiles
     */
    pub fn get_vertex_between_tiles_coord(q: Coordinate2, r: Coordinate2, s: Coordinate2) -> Coordinate3 {
        
        // Find A
        let (a, u1, u2) = if q.y == r.y {
            (s, q, r)
        } else if r.y == s.y {
            (q, r, s)
        } else if s.y == q.y {
            (r, s, q)
        } else {
            panic!("Invalid tile coordinates");
        };

        // Find primeness of shape
        let prime = a.y < u1.y;

        // Get B and C
        let result = if prime {
            let (b, c) = if a.x == u1.x {
                (u1, u2)
            } else {
                (u2, u1)
            };
            Coordinate3::new(a.x, b.y, -c.x - c.y)
        } else {
            let (b, c) = if a.x == u1.y {
                (u2, u1)
            } else {
                (u1, u2)
            };
            Coordinate3::new(c.x, b.y, -a.x - a.y)
        };

        if result.sum().abs() != 1 {
            panic!("Invalid tile coordinates provided");
        }

        result
    }

}


pub trait VectorOps {
    fn sum(&self) -> i8;
    fn abs_max(&self) -> i8;
    fn abs_sum(&self) -> i8;
}

#[derive(Clone, Debug)]
pub struct Coordinate2 {
    pub x: i8,
    pub y: i8,
}

impl Coordinate2 {
    pub fn new(x: i8, y: i8) -> Self {
        Coordinate2 { x, y }
    }

    /**
     * Check if a tile coordinate is valid for this board
     */
    pub fn is_valid_tile(&self) -> bool {
        if self.x.abs() > I8_R || self.y.abs() > I8_R || (-self.x - self.y).abs() > I8_R  {
            return false
        }
        return true
    }

}

impl VectorOps for Coordinate2 {
    fn sum(&self) -> i8 {
        return self.x + self.y
    }

    fn abs_max(&self) -> i8 {
        i8::max(self.x.abs(), self.y.abs())
    }

    fn abs_sum(&self) -> i8 {
        self.x.abs() + self.y.abs()
    }
}


impl PartialEq for Coordinate2 {

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    
}

#[derive(Debug)]
pub struct Coordinate3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Coordinate3 {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Coordinate3 { x, y, z }
    }

    /**
     * Utility to determine if a Vertex is on the game board
     */
    pub fn is_valid_vertex(&self) -> bool {
        self.abs_sum() <= 2 * I8_R + 1
    }

    /**
     * Utility to determine if an Edge coordinate is on the game board
     */
    pub fn is_valid_edge(&self) -> bool {
        self.x >= 0 && self.x <= 2
            && self.y >= -I8_R && self.y <= I8_R + 1
            && self.z >= -I8_R && self.z <= I8_R + 1
    }
}

impl VectorOps for Coordinate3 {
    fn sum(&self) -> i8 {
        return self.x + self.y + self.z
    }

    fn abs_max(&self) -> i8 {
        i8::max(i8::max(self.x.abs(), self.y.abs()), self.y.abs())
    }
    
    fn abs_sum(&self) -> i8 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl PartialEq for Coordinate3 {

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

}