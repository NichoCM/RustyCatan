use super::{tile::Tile, Board};

impl Board {

    /**
     * Convert the board to notation
     */
    pub fn as_notation(self: &Board) -> String {
        let mut value  = String::new();
        Self::get_tile_coords().iter().for_each(|coord| {
            let (tile, v) = Tile::from_u8(self.get_tile(coord));
            value.push_str(format!("{}{}", tile.as_notation(), v).as_str());
        });
        value
    }

    /**
     * Create a board from notation
     */
    pub fn from_notation(str: String) -> Board {
        let mut board = Board::new();

        let mut character = 0;
        let mut tile = Tile::Empty;
        let mut value: u8 = 0;

        let mut tiles: Vec<(Tile, u8)> = Vec::new();

        str.chars().into_iter().for_each(|c| {
            match tile {
                Tile::Empty => {
                    tile = Tile::from_string(&c.to_string());
                },
                _ => {
                    if c.is_numeric() {
                        value = c.to_digit(10).unwrap() as u8 + value * 10;
                        if Board::is_tile_value(value) {
                            tiles.push((tile, value));
                            tile = Tile::Empty;
                            value = 0;
                        } else if value != 1 {
                            panic!("Invalid character in board notation. Character:  {}", character);
                        }
                    } else {
                        panic!("Invalid character in board notation. Character:  {}", character);
                    }
                }
            }
            character += 1;
        });
        
        let coords =  Self::get_tile_coords();
        if tiles.len() != coords.len() {
            panic!("Invalid tile length for Board notation");
        }

        coords.iter().zip(tiles.into_iter()).for_each(|(coord, (tile, value)) | {
            board.set_tile(coord, tile, value);
        });

        board
    }

}