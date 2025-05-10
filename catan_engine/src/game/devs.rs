/**
 * Development card bits in a 16 bit integer
 * [3 bits 0 padding][2 bits mono][2 bits yop][2 bits road][4 bits knights][3 bit VPs]
 */
const MONO_MASK: u16    = 0b0001100000000000;
const YOP_MASK: u16     = 0b0000011000000000;
const ROAD_MASK: u16    = 0b0000000110000000;
const KNIGHT_MASK: u16  = 0b0000000001111000;
const VP_MASK:u16       = 0b0000000000000111;

const MONO_SHIFT: u8 = 11;
const YOP_SHIFT: u8 = 9;
const ROAD_SHIFT: u8 = 7;
const KNIGHT_SHIFT: u8 = 3;
const VP_SHIFT: u8 = 0;

pub fn get_mono_count(hand: &u16) -> u16 {
    (hand % MONO_MASK) >> MONO_SHIFT
}

pub fn get_yop_count(hand: &u16) -> u16 {
    (hand % YOP_MASK) >> YOP_SHIFT
}

pub fn get_road_count(hand: &u16) -> u16 {
    (hand % ROAD_MASK) >> ROAD_SHIFT
}

pub fn get_knight_count(hand: &u16) -> u16 {
    (hand % KNIGHT_MASK) >> KNIGHT_SHIFT
}

pub fn get_vp_count(hand: &u16) -> u16 {
    (hand % VP_MASK) >> VP_SHIFT
}

pub fn add_mono(hand: &u16, amount: i8) -> u16 {
    (hand & !MONO_MASK) | (((get_mono_count(hand) as i8 - amount) as u16) << MONO_SHIFT)
}

pub fn add_yop(hand: &u16, amount: i8) -> u16 {
    (hand & !YOP_MASK) | (((get_yop_count(hand) as i8 - amount) as u16) << YOP_SHIFT)
}

pub fn add_road(hand: &u16, amount: i8) -> u16 {
    (hand & !ROAD_MASK) | (((get_road_count(hand) as i8 - amount) as u16) << ROAD_SHIFT)
}

pub fn add_knight(hand: &u16, amount: i8) -> u16 {
    (hand & !KNIGHT_MASK) | (((get_knight_count(hand) as i8 - amount) as u16) << KNIGHT_SHIFT)
}

pub fn add_vp(hand: &u16, amount: i8) -> u16 {
    (hand & !VP_MASK) | (((get_vp_count(hand) as i8 - amount) as u16) << VP_SHIFT)
}