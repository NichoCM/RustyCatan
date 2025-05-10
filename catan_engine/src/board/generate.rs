use rand::distributions::Alphanumeric;
use std::vec;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;

use crate::board::coordinate::Coordinate2;
use crate::board::I8_R;
use crate::board::USIZE_R;

use super::{Board, Tile, tile};

const SHEEP_FREQUENCY: u8 = 4;
const WHEAT_FREQUENCY: u8 = 4;
const LUMBER_FREQUENCY: u8 = 4;
const BRICK_FREQUENCY: u8 = 3;
const ORE_FREQUENCY: u8 = 3;
const DESERT_FREQUENCY: u8 = 1;

impl Board {
        /**
     * Create a random board without providing a seed
     */
    pub fn random() -> Board {
        let seed = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        Self::random_from_seed(seed)
    }

    /**
     * Generate a random board
     */
    pub fn random_from_seed(seed: String) -> Board {

        let rng: &mut Pcg64 = &mut Seeder::from(&seed).make_rng();
        
        let mut bag = Vec::<Tile>::new();

        // Add the tiles
        for _ in 0 .. SHEEP_FREQUENCY {
            bag.push(Tile::Sheep);
        }

        for _ in 0 .. WHEAT_FREQUENCY {
            bag.push(Tile::Wheat);
        }

        for _ in 0 .. LUMBER_FREQUENCY {
            bag.push(Tile::Lumber);
        }

        for _ in 0 .. BRICK_FREQUENCY {
            bag.push(Tile::Brick);
        }

        for _ in 0 .. ORE_FREQUENCY {
            bag.push(Tile::Ore);
        }

        for _ in 0 .. DESERT_FREQUENCY {
            bag.push(Tile::Desert);
        }

        // Shuffle the bag
        bag.shuffle(rng);
        
        // Create a new board
        let mut board = Board::new();
        let mut valid_coords = Self::get_tile_coords();

        // Set the tiles randomly
        valid_coords.iter().for_each(|coord| {
            let tile = bag.pop().unwrap();
            board.tiles[(coord.x + I8_R) as usize][(coord.y + I8_R) as usize] = tile.as_value() << 4
        });

        // Create a bag of valid tile values and shuffle
        let mut value_bag: Vec<u8> = vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        value_bag.shuffle(rng);

        // Look ahead step for the back tracking algorithm to ensure 2 tile values
        // are not adjacent 
        fn next(board: &mut Board, coords: &mut Vec<Coordinate2>, values: &mut Vec<u8>) -> bool {
            // Values may run out before the 
            if coords.is_empty() {
                return true
            }

            let coord = coords.pop().unwrap();
            let (tile, _) = Tile::from_u8(board.get_tile(&coord));

            // Desert tile does not need a value
            if tile == Tile::Desert {
                board.robber = coord.clone();
                if next(board, coords, values) {
                    return true;
                }
                board.robber = Coordinate2::new(0, 0);
                coords.push(coord);
                return false
            }

            // Store visited values to prevent duplicate visits
            let mut visited: [bool; 13] = [false; 13];

            // Attempt each value in the bag once
            for i in 0 .. values.len() {
                let value = values[i];
                let is_value_6_8 = value == 6 || value == 8;

                // Prevent duplicate visits
                if visited[value as usize] {
                    continue;
                }
                visited[value as usize] = true;

                let neighbors = Board::get_tile_neighbor_coords(&coord);
                let mut valid = true;

                // Check if the neighbors are valid
                for coord in neighbors {
                    let tile_value = board.get_tile(&coord) & tile::TILE_VALUE_MASK;
                    let is_6_8 = tile_value == 6 || tile_value == 8;

                    // Check if the valeus are equal, or if 6 and 8 are adjacent
                    if tile_value == value || (is_value_6_8 && is_6_8) {
                        valid = false;
                        break;
                    }
                }
                
                // If neighbor check is valid, perform the next step
                if valid {
                    let v = values.remove(i);
                    board.set_tile(&coord, tile, value);
                    if next(board, coords, values)  {
                        return true
                    }
                    values.insert(i, v);
                }
            }
        
            // Revert the tile and readd the coordinate to the list
            board.set_tile(&coord, tile, 0);
            coords.push(coord);

            false
        }

        next(&mut board, &mut valid_coords, &mut value_bag);

        board
        
    }
}