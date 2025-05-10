use crate::board::tile::{BRICK_VALUE, LUMBER_VALUE, ORE_VALUE, SHEEP_VALUE, WHEAT_VALUE};

const CHUNK_SIZE: u8 = 6;
const MASK: u32 = 0b111111;
/**
 * Cards in player's hand
 */
const BRICK_SHIFT: u8   = (BRICK_VALUE - 1) * CHUNK_SIZE;
const LUMBER_SHIFT: u8  = (LUMBER_VALUE - 1) * CHUNK_SIZE;
const ORE_SHIFT: u8     = (ORE_VALUE - 1) * CHUNK_SIZE;
const SHEEP_SHIFT: u8   = (SHEEP_VALUE - 1) * CHUNK_SIZE;
const WHEAT_SHIFT: u8   = (WHEAT_VALUE - 1) * CHUNK_SIZE;

const BRICK_MASK: u32   = MASK << BRICK_SHIFT;
const LUMBER_MASK: u32  = MASK << LUMBER_SHIFT;
const ORE_MASK: u32     = MASK << ORE_SHIFT;
const SHEEP_MASK: u32   = MASK << SHEEP_SHIFT;
const WHEAT_MASK: u32   = MASK << WHEAT_SHIFT;

pub const ROAD_RECIPE: u32 = (1 << BRICK_SHIFT) | (1 << LUMBER_SHIFT);
pub const DEV_RECIPE: u32 = (1 << ORE_SHIFT) | (1 << WHEAT_SHIFT) | (1 << SHEEP_SHIFT);
pub const SETTLE_RECIPE: u32 = (1 << BRICK_SHIFT) | (1 << LUMBER_SHIFT) | (1 << WHEAT_SHIFT) | (1 << SHEEP_SHIFT);
pub const CITY_RECIPE: u32 = (3 << ORE_SHIFT) | (2 << WHEAT_SHIFT);

// Maximum allowed number of cards in circulation
pub const MAX_CARDS: u32 = 19;

// MARK - helper utility functions with explicit names
pub fn get_brick_count(hand: u32) -> u32 {
    (hand & BRICK_MASK) >> BRICK_SHIFT
}

pub fn get_lumber_count(hand: u32) -> u32 {
    (hand & LUMBER_MASK) >> LUMBER_SHIFT
}

pub fn get_ore_count(hand: u32) -> u32 {
    (hand & ORE_MASK) >> ORE_SHIFT
}

pub fn get_sheep_count(hand: u32) -> u32 {
    (hand & SHEEP_MASK) >> SHEEP_SHIFT
}

pub fn get_wheat_count(hand: u32) -> u32 {
    (hand & WHEAT_MASK) >> WHEAT_SHIFT
}

pub fn add_brick(hand: u32, amount: i8) -> u32 {
    (hand & !BRICK_MASK) | (((get_brick_count(hand) as i8 + amount) as u32) << BRICK_SHIFT)
}

pub fn add_lumber(hand: u32, amount: i8) -> u32 {
    (hand & !LUMBER_MASK) | (((get_lumber_count(hand) as i8 + amount) as u32) << LUMBER_SHIFT)
}

pub fn add_ore(hand: u32, amount: i8) -> u32 {
    (hand & !ORE_MASK) | (((get_ore_count(hand) as i8 + amount) as u32) << ORE_SHIFT)
}

pub fn add_sheep(hand: u32, amount: i8) -> u32 {
    (hand & !SHEEP_MASK) | (((get_sheep_count(hand) as i8 + amount) as u32) << SHEEP_SHIFT)
}

pub fn add_wheat(hand: u32, amount: i8) -> u32 {
    (hand & !WHEAT_MASK) | (((get_wheat_count(hand) as i8 + amount) as u32) << WHEAT_SHIFT)
}

// MARK - controls over the hand without having to know the resource

/**
 * Add a resource to a hand using the resource ID
 * 
 * This is implemented for efficiency
 */
pub fn add(hand: u32, res: u8, amount: i8) -> u32 {
    let shift: u8 = (res - 1) * CHUNK_SIZE;
    let mask: u32 = MASK << shift;
    let masked_hand = hand & !mask;
    let value = (hand & mask) >> shift;
    masked_hand | (((value as i8 + amount) as u32) << shift)
}

/**
 * Get the resource count using the resource ID
 * 
 * This is implemented for efficiency
 */
pub fn count(hand: u32, res: u8) -> u32 {
    let shift: u8 = (res - 1) * CHUNK_SIZE;
    let mask = MASK << shift;
    (hand & mask) >> shift
}

/**
 * Get the number of cards in a hand
 */
pub fn size(hand: u32) -> u32 {
    (hand & MASK)
    + ((hand >> CHUNK_SIZE * 1) & MASK)
    + ((hand >> CHUNK_SIZE * 2) & MASK)
    + ((hand >> CHUNK_SIZE * 3) & MASK)
    + ((hand >> CHUNK_SIZE * 4) & MASK)
}

/**
 * Check if a hand contains the cards in another hand
 */
pub fn has(hand: u32, has: u32) -> bool {
    for i in 1 ..= 5 {
        if count(hand, i) < count(has, i) {
            return false
        }
    }
    true
}

/**
 * Subtract values of one hand from another
 */
pub fn subtract(hand: u32, value: u32) -> u32 {
    let mut new = 0;
    for i in 0 .. 5 {
        let shift = CHUNK_SIZE * i;
        new |= ((hand >> shift) & MASK) - ((value >> shift) & MASK) << shift
    }
    new
}

/**
 * Utility function to help format the binary representation
 * oh a hand (or othER BINARY ENCODED VALUES)
 */
pub fn format_binary_u32(i: u32, n: usize, p: usize) -> String {
    let format = format!("{:032b}", i);
    let padding: String = format.chars().take(p).collect();
    let format: String = format.chars().skip(p).collect();
    let chunked = format.chars()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    format!("{} {}", padding, chunked)
}