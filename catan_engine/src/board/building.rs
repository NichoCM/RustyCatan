pub const PLAYER_VALUE_MASK: u8 = 0b00001111;
pub const BUILDING_SHIFT: u8 = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Building {
    Settlement, City, Empty
}

impl Building {

    pub fn as_notation(&self) -> String {
        match self {
            Building::Settlement => "S".to_string(),
            Building::City => "C".to_string(),
            _ => unimplemented!("Invalid building notation"),
        }
    }

    pub fn from_string(c: &String) -> Building {
        match c.as_str() {
            "S" => Building::Settlement,
            "C" => Building::City,
            _ => unimplemented!("Invalid building type"),
        }
    }
    
    pub fn as_value(&self) -> u8 {
        match self {
            Building::Empty => 0,
            Building::Settlement => 1,
            Building::City => 2,
        }
    }

    pub fn from_u8(building: u8) -> (Building, u8) {
        let p = building & PLAYER_VALUE_MASK;
        let b = building >> 4;
        match b {
            0 => (Building::Empty, 0),
            1 => (Building::Settlement, p),
            2 => (Building::City, p),
            _ => unimplemented!()
        }
    }

}