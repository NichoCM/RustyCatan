
mod generate;
mod notation;
pub mod display;
pub mod building;
pub mod coordinate;
pub mod tile;

use building::{Building, PLAYER_VALUE_MASK};
use coordinate::{Coordinate2, Coordinate3};
use tile::Tile;

const USIZE_R: usize = 2;
const I8_R:i8 = USIZE_R as i8;

pub const PORT_LOCATIONS: [(i8, i8, i8); 9] = [
    (0, -3, 3),
    (1, -3, 3),
    (2, -3, 3),
    (0, 1, -2),
    (1, 1, -2),
    (2, 1, -2),
    (0, 2, -1),
    (1, 2, -1),
    (2, 2, -1),
];

pub struct Board {
    pub tiles: [[u8; USIZE_R * 2 + 1]; USIZE_R * 2 + 1],
    pub roads: [[[u8; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1]; 3],
    pub buildings: [[[u8; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1],
    pub ports: [u8; 9], // 9 port spots be default, will need to change this to support custom port locations
    pub robber: Coordinate2,
}

impl Board {
    
    /**
     * Set a tile in the grid from the Tile enum and a value. This is encoded into a 1 byte representation
     * of the tile and value.
     */
    pub fn set_tile(&mut self, coord: &Coordinate2, tile: Tile, value: u8) {
        if coord.is_valid_tile() && Self::is_tile_value(value) {
            self.tiles[(coord.x + I8_R) as usize][(coord.y + I8_R) as usize] = tile.as_value() << 4 | value;
        } else {
            panic!("Invalid coordinates while setting tile. {} with value {}", coord, value);
        }
    }

    /**
     * Returns the u16 encoded form of a tile, defined as follows
     * 
     * [4 bits: Tile type][4 bits: Tile Value]
     */
    pub fn get_tile(&self, coord: &Coordinate2) -> u8 {
        if coord.is_valid_tile() {
            return self.tiles[(coord.x + I8_R)as usize][(coord.y + I8_R) as usize]
        }
        panic!("Invalid coordinates while getting tile");
    }

    /**
     * Returns the player 1 - 4 if they own the road, or 0 if there is no road
     */
    pub fn get_road(&self, axis: i8, coord: &Coordinate2) -> u8 {
        return self.roads[axis as usize][(coord.x + I8_R + 1) as usize][(coord.y + I8_R + 1) as usize]
    }

    pub fn set_road(&mut self, coord: &Coordinate3, player: u8) {
        self.roads[coord.x as usize][(coord.y + I8_R + 1) as usize][(coord.z + I8_R + 1) as usize] = player;
    }

    /**
     * Returns the encoded road data for a specific vertex
     */
    pub fn get_building(&self, coord: &Coordinate3) -> u8 {
        return self.buildings[(coord.x + I8_R + 1) as usize][(coord.y + I8_R + 1) as usize][(coord.z + I8_R + 1) as usize]
    }

    /**
     * Set a building
     */
    pub fn set_building(&mut self, coord: &Coordinate3, building: Building, player: u8) {
        if building == Building::Empty {
            self.buildings[(coord.x + I8_R + 1) as usize][(coord.y + I8_R + 1) as usize][(coord.z + I8_R + 1) as usize] = 0;
        } else {
            self.buildings[(coord.x + I8_R + 1) as usize][(coord.y + I8_R + 1) as usize][(coord.z + I8_R + 1) as usize] = building.as_value() << 4 | player;
        }
    }

    /**
     * Check if the specified tile value is valid
     * 
     * Note: 0 is considered valid to represent tiles without a value (e.g desert)
     */
    pub fn is_tile_value(v: u8) -> bool {
        return (v >= 2 && v <= 12) || v == 0
    }

    /**
     * Create a new blank board
     */
    pub fn new() -> Self {
        Board {
            tiles: [[0; USIZE_R * 2 + 1]; USIZE_R * 2 + 1],
            roads: [[[0; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1]; 3],
            buildings: [[[0; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1]; (USIZE_R + 1) * 2 + 1],
            robber: Coordinate2::new(0, 0),
            ports: [0; 9],
        }
    }

    /**
     * Get all the tiles (as u8 encoded tiles) around a vertex 
     */
    pub fn get_vertex_adjacent_tiles(&self, coord: &Coordinate3) -> Vec<u8> {
        Self::get_vertex_adjacent_tile_coords(coord).iter().filter(|e| {
           e.is_valid_tile()
        }).map(|e| {
            self.get_tile(e)
        }).collect()
    }

    /**
     * Get valid building vertex coordinates for the whole board
     */
    pub fn get_valid_starting_buiding_coords(&self) -> Vec<Coordinate3> {
        Self::get_vertex_coords().into_iter().filter(|e| {
            self.is_valid_building_coord(&e)
        }).collect()
    }

    /**
     * Detect if the coordinate can accept a building, which much be
     * no less that 2 edges away from another building
     */
    pub fn is_valid_building_coord(&self, coord: &Coordinate3) -> bool {

        if self.has_building(coord) {
            return false;
        }

        // Get the adjoining vertex in all directions
        for d in 0 ..= 2 {
            let v: Coordinate3 = Self::get_prime_pair_vertex(d, coord);
            if v.is_valid_vertex() && self.has_building(&v) {
                return false
            }
        }
        true
    }

    /**
     * Determine if a Vertex has a building
     */
    fn has_building(&self, coord: &Coordinate3) -> bool {
        self.get_building(coord) & PLAYER_VALUE_MASK != 0
    }

    /**
     * During initial placement, get the road spots around a settlement
     */
    pub fn get_placement_roads_around(&self, coord: &Coordinate3) -> Vec<Coordinate3> {
        Self::get_vertex_adjacent_edge_coords(coord).into_iter().filter(|e| {
            e.is_valid_edge() && !self.has_road(e.x, &Coordinate2::new(e.y, e.z))
        }).collect()
    }

    /**
     * Determine if an edge has a road
     */
    fn has_road(&self, dir: i8, coord: &Coordinate2) -> bool {
        self.get_road(dir, coord) & PLAYER_VALUE_MASK != 0
    }

}