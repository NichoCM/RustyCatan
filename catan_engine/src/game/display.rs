use std::fmt::format;

use colored::{Color, ColoredString, Colorize};

use crate::{board::display::player_color, game::{devs, hand}};

use super::Game;

impl Game {

    pub fn print(&self) {
        let points = self.get_points();
        // Print the hands
        for p in 1 ..= self.player_count {
            let color = player_color(p);
            let hand = self.get_hand(p);
            let devs = self.get_devs(p);
            let horiz_line = format!("+---------------------------------------+").color(color);
            let vert_line = format!("|").color(color);
            println!("{}", horiz_line);
            print!("{} ", vert_line);
            print!("{}                         {:<2}VP", format!("Player {}", p).color(color), points[(p - 1) as usize]);
            println!(" {}", vert_line);
            println!("{} ", horiz_line);
            print!("{} ", vert_line);
            print!("{}", Self::player_hand(hand));
            println!(" {}", vert_line);
            println!("{}", horiz_line);
            print!("{} ", vert_line);
            print!("{}", Self::player_devs(p, devs));
            println!(" {}", vert_line);
            println!("{}", horiz_line);
        }
        self.board.print();
    }

    fn player_hand(hand: u32) -> String {
        let b = hand::get_brick_count(hand);
        let l = hand::get_lumber_count(hand);
        let o = hand::get_ore_count(hand);
        let s = hand::get_sheep_count(hand);
        let w = hand::get_wheat_count(hand);
        format!("{}: {:>2} | {}: {:>2} | {}: {:>2} | {}: {:>2} | {}: {:>2}",
            "B".color(Color::Red),
            b.to_string().color(Color::White),
            "L".color(Color::Green),
            l.to_string().color(Color::White),
            "O".color(Color::BrightBlack),
            o.to_string().color(Color::White),
            "S".color(Color::BrightGreen),
            s.to_string().color(Color::White),
            "W".color(Color::Yellow),
            w.to_string().color(Color::White),
        )
    }

    
    fn player_devs(player: u8, hand: u16) -> String {
        let k = devs::get_knight_count(&hand);
        let r = devs::get_road_count(&hand);
        let y = devs::get_yop_count(&hand);
        let m = devs::get_mono_count(&hand);
        let v = devs::get_vp_count(&hand);
        format!("{}: {:>2} | {}: {:>2} | {}: {:>2} | {}: {:>2} | {}: {:>2}",
            "K".color(Color::Magenta),
            k.to_string().color(Color::White),
            "R".color(Color::Magenta),
            r.to_string().color(Color::White),
            "Y".color(Color::Magenta),
            y.to_string().color(Color::White),
            "M".color(Color::Magenta),
            m.to_string().color(Color::White),
            "V".color(Color::Magenta),
            v.to_string().color(Color::White),
        )
    }

}