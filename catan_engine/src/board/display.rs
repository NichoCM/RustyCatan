use std::fmt::Display;
use colored::{Color, Colorize};
use crate::board::{building::Building, coordinate::{Coordinate2, X_EDGE, Y_EDGE, Z_EDGE}, tile::Tile, I8_R, PORT_LOCATIONS, USIZE_R};
use super::{coordinate::Coordinate3, Board};

impl Board {
    
    /**
     * Disgusting code to print the board.
     * I never want to look at this again.
     */
    pub fn print(&self) {

        fn pad(y: i8) {
            for _ in 0 ..= y.abs() {
                print!("   ");
            } 
        }

        println!();
        for y in -I8_R ..= I8_R {

            // Print Z and X axis roads
            if y <= 0 {

                // House row
                pad(y);
                print!(" ");
                for x in -I8_R - y ..= I8_R  {
                    // Prime vertices
                    let y = y - 1;
                    let z = -1 - x - y;
                    let b = self.get_building(&Coordinate3::new(x, y, z));
                    let (building, player) = Building::from_u8(b);

                    if player == 0 {
                        print!("   {}  ", "x");
                    } else {
                        print!("   {}  ", building.as_notation().color(player_color(player)));
                    }
                }
                println!();

                pad(y);
                print!("  ");
                for x in -I8_R ..= I8_R {
                    let coord: Coordinate2 = Coordinate2::new(x, y);
                    let z: i8 = -x - y;
                    if coord.is_valid_tile() {
                        let road =  self.get_road(X_EDGE, &Coordinate2::new(y - 1, z + 1));
                        let x_color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!("{}", "/".color(x_color).bold());
                        print!("   ");
                        let road =  self.get_road(Z_EDGE, &Coordinate2::new(x, y));
                        let z_color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!("{} ", "\\".color(z_color).bold());
                    }
                }

                println!();

                // House row
                pad(y);
                print!(" ");
                for x in -I8_R - y ..= I8_R + 1  {
                    // Not prime vertices
                    let z = 1 - x - y;
                    let b = self.get_building(&Coordinate3::new(x, y, z));
                    let (building, player) = Building::from_u8(b);

                    if player == 0 {
                        print!("{}     ", "x");
                    } else {
                        print!("{}     ", building.as_notation().color(player_color(player)));
                    }
                }
            }

            println!();
            
            pad(y);

            for x in -I8_R ..= I8_R {
                let z = -x - y;
                let coord: Coordinate2 = Coordinate2::new(x, y);
                if coord.is_valid_tile() {

                    // Print next roads
                    let road =  self.get_road(Y_EDGE, &Coordinate2::new(-y - x, x));
                    let color = if road != 0 {
                        player_color(road)
                    } else {
                        Color::BrightBlack
                    };
                    print!(" {} ", "|".color(color).bold());

                    let (tile, value) = Tile::from_u8(self.get_tile(&coord));
                    let str = format!("{}{}", tile.as_notation().color(tile.get_color()), format!("{:>2}", value).white());
                    if self.robber == coord {
                        print!("{}", str.strikethrough());
                    } else {
                        print!("{}", str);
                    }

                    if x == I8_R {
                        let road =  self.get_road(Y_EDGE, &Coordinate2::new(z - 1, x + 1));
                        let color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!(" {}", "|".color(color).bold());
                    } else if z == -I8_R {
                        let road =  self.get_road(Y_EDGE, &Coordinate2::new(z - 1, x + 1));
                        let color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!(" {}", "|".color(color).bold());
                    }
                }

            }
            println!();
            if y >= 0 {
                
                // House row
                pad(y);
                print!(" ");
                for x in -I8_R - 1 ..= I8_R - y  {
                    // Prime vertices
                    let z = -1 - x - y;
                    let b = self.get_building(&Coordinate3::new(x, y, z));
                    let (building, player) = Building::from_u8(b);

                    if player == 0 {
                        print!("{}     ", "x");
                    } else {
                        print!("{}     ", building.as_notation().color(player_color(player)));
                    }
                }
                println!();

                pad(y);
                print!("  ");
                for x in -I8_R ..= I8_R {
                    let coord: Coordinate2 = Coordinate2::new(x, y);
                    let z = -y - x;
                    if coord.is_valid_tile() {
                        let road =  self.get_road(Z_EDGE, &Coordinate2::new(x - 1, y + 1));
                        let x_color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!("{}", "\\".color(x_color).bold());
                        print!("   ");
                        let road =  self.get_road(X_EDGE, &Coordinate2::new(y, z));
                        let z_color = if road != 0 {
                            player_color(road)
                        } else {
                            Color::BrightBlack
                        };
                        print!("{} ", "/".color(z_color).bold());
                    }
                }

                println!();

                // House row
                pad(y);
                print!(" ");
                for x in -I8_R ..= I8_R - y {
                    // Prime vertices
                    let y = y + 1;
                    let z = 1 - x - y;
                    let b = self.get_building(&Coordinate3::new(x, y, z));
                    let (building, player) = Building::from_u8(b);

                    if player == 0 {
                        print!("   {}  ", "x");
                    } else {
                        print!("   {}  ", building.as_notation().color(player_color(player)));
                    }
                }
            }
        }
        println!();
    }
}

impl Display for Board {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not implemented")
    }

}

impl Display for Coordinate2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
} 


impl Display for Coordinate3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Tile {

    fn get_color(&self) -> Color {
        match self {
            Tile::Brick => Color::Red,
            Tile::Wheat => Color::Yellow,
            Tile::Desert => Color::BrightYellow,
            Tile::Ore => Color::BrightBlack,
            Tile::Sheep => Color::BrightGreen,
            Tile::Empty => Color::Blue,
            Tile::Lumber => Color::Green,
            _ => unimplemented!(),
        }
    }

}

pub fn player_color(player: u8) -> Color {
    match player {
        1 => Color::Blue,
        2 => Color::Red,
        3 => Color::Yellow,
        4 => Color::Green,
        _ => unimplemented!(),
    }
}