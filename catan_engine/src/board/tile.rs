pub const TILE_SHIFT: u8 = 4;
pub const TILE_VALUE_MASK: u8 = 0b00001111;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Wheat, Sheep, Lumber, Ore, Brick, Desert, Empty
}

pub const BRICK_VALUE: u8   = 1;
pub const LUMBER_VALUE: u8  = 2;
pub const ORE_VALUE: u8     = 3;
pub const SHEEP_VALUE: u8   = 4;
pub const WHEAT_VALUE: u8   = 5;
pub const DESERT_VALUE: u8  = 6;

impl Tile {

    pub fn as_notation(&self) -> String {
        match self {
            Tile::Empty => "?".to_string(),
            Tile::Brick => "B".to_string(),
            Tile::Desert => "D".to_string(),
            Tile::Lumber => "L".to_string(),
            Tile::Ore => "O".to_string(),
            Tile::Sheep => "S".to_string(),
            Tile::Wheat => "W".to_string(),
        }
    }

    pub fn from_string(c: &String) -> Tile {
        match c.as_str() {
            "B" => Tile::Brick,
            "D" => Tile::Desert,
            "L" => Tile::Lumber,
            "O" => Tile::Ore,
            "S" => Tile::Sheep,
            "W" => Tile::Wheat,
            _ => unimplemented!("Invalid tile type")
        }
    }
    
    pub fn as_value(&self) -> u8 {
        match self {
            Tile::Empty => 0,
            Tile::Brick => BRICK_VALUE,
            Tile::Lumber => LUMBER_VALUE,
            Tile::Ore => ORE_VALUE,
            Tile::Sheep => SHEEP_VALUE,
            Tile::Wheat => WHEAT_VALUE,
            Tile::Desert => DESERT_VALUE,
        }
    }

    pub fn from_u8(tile: u8) -> (Tile, u8) {
        let v = tile & TILE_VALUE_MASK;
        let t = tile >> TILE_SHIFT;
        match t {
            0 => (Tile::Empty, 0),
            BRICK_VALUE => (Tile::Brick, v),
            LUMBER_VALUE => (Tile::Lumber, v),
            ORE_VALUE => (Tile::Ore, v),
            SHEEP_VALUE => (Tile::Sheep, v),
            WHEAT_VALUE => (Tile::Wheat, v),
            DESERT_VALUE => (Tile::Desert, 0),
            _ => unimplemented!()
        }
    }

}