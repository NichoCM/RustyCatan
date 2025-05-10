mod devs;
pub mod hand;
mod action;
mod display;

use std::io::stdin;

use action::{GameAction, PlayerAction};
use rand::Rng;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::board::{building::{BUILDING_SHIFT, PLAYER_VALUE_MASK}, Board};

pub struct Game {
    player_count: u8,
    pub board: Board,
    hands: [u32; 4],
    devs: [u16; 4],
    played_knights: [u8; 4],
    largest_army_player: u8,
    longest_road_player: u8,
    max_cards: u8,
    orbit: u8,
}

impl Game {

    pub fn get_points(&self) -> [u8; 4] {
        let mut points: [u8; 4] = [0; 4];

        // Board points
        self.board.buildings.iter().for_each(|a| {
            a.iter().for_each(|b| {
                b.iter().for_each(|building| {
                    let player = building & PLAYER_VALUE_MASK;
                    if building & PLAYER_VALUE_MASK != 0 {
                        points[(player - 1) as usize] += building >> BUILDING_SHIFT
                    }
                })
            });
        });

        // VP devs
        self.devs.iter().enumerate().for_each(|(player, hand)| {
            points[player] += devs::get_vp_count(hand) as u8
        });

        // Largest army
        if self.largest_army_player != 0 {
            points[(self.largest_army_player - 1) as usize] += 1;
        }

        // Longest road
        if self.longest_road_player != 0 {
            points[(self.longest_road_player - 1) as usize] += 1;
        }
        
        points
    }

    /**
     * Check if this game is in a winning state
     */
    pub fn has_winner(&self) -> bool {
        self.get_points().into_iter().max().unwrap() == 10
    }


    /**
     * The the encoded hand of a player
     */
    pub fn get_hand(&self, player: u8) -> u32 {
        self.hands[(player - 1) as usize]
    }

    /**
     * Update a player's hand
     */
    pub fn set_hand(&mut self, player: u8, hand: u32) {
        self.hands[(player - 1) as usize] = hand
    }

    /**
     * The encoded development cards of a player
     */
    pub fn get_devs(&self, player: u8) -> u16 {
        self.devs[(player - 1) as usize]
    }

    pub fn start(&mut self) {
        let rng = &mut thread_rng();
        let mut action = GameAction::GameStart;

        // Phase for inital placements
        while action != GameAction::PlacementFinished {
            action = self.perform_game_action(rng, action);
        }

        let mut player = 1;
        while action != GameAction::Finished {
            if self.has_winner() {
                break;
            }
            let roll = GameAction::Roll(player, Self::roll(rng));
            action = self.perform_game_action(rng, roll);
            player += 1;
            if player > self.player_count {
                player = 1;
            }
            self.print();
        }

        println!("Game over");
    }

    /**
     * Roll the dice
     */
    fn roll(rng: &mut ThreadRng) -> u8 {
        let d1 = rng.gen_range(1 ..= 6);
        let d2 = rng.gen_range(1 ..= 6);
        d1 + d2
    }

    /**
     * Steal from a hand at random. This does not update
     * the hand at all, it just returns a valid resource
     * to steal
     */
    fn steal(hand: u32, rng: &mut ThreadRng) -> u8 {
        let size = hand::size(hand);
        let mut i = rng.gen_range(0 .. size);
        for res in 1 ..= 5 {
            let count = hand::count(hand, res);
            if count > i {
                return res
            }
            i -= count;
        }
        panic!("Failed to steal from hand - this is an implementation error");
    }

    /**
     * Provide a list of actions to a player agent and select a move to play
     */
    pub fn call_player(&mut self, rng: &mut ThreadRng, player: u8, actions: Vec<PlayerAction>) {
        // If there are no actions to choose from, let this function return. Otherwise ask the player
        // agent for a move
        let mut actions = actions;
        while !actions.is_empty() {
            println!("Actions: {}", actions.len());
            // TODO - add player agent trait. For now we just pick a random move
            actions.shuffle(&mut thread_rng());
            let pop = actions.pop().unwrap();
            actions = self.perform_player_action(rng, player, pop);
            stdin().read_line(&mut String::new());
        }
    }

    /**
     * Ask the agent to discard cards from a player's hand after a "7 out"
     */
    pub fn call_player_discard(&mut self, player: u8) -> u32 {
        // TODO - call the user agent, this will randomly discard cards.
        let mut hand = self.get_hand(player);
        let size = hand::size(hand);
        let mut discard_count = size / 2;
        while discard_count > 0 {
            let res: u8 = thread_rng().gen_range(1 ..= 5);
            if hand::count(hand, res) > 0 {
                discard_count -= 1;
                hand = hand::add(hand, res, -1);
            }
        }
        hand
    }

    /**
     * Ask the agent to decide which player to rob.
     * 
     * Return the index of which player to rob from the options list.
     * This must be a valid player, where the value of options[i] == true.
     * 
     * Typically we index players starting from 1, in this case player 1 is 0.
     */
    pub fn call_player_rob(&mut self, options: [bool; 4]) -> usize {
        // TODO - call the user agent, this is just randomnly picking a player to rob
        let mut players = options.into_iter()
            .enumerate()
            .filter(|(_, b) | { *b }).map(|(i, _)| { i })
            .collect::<Vec<usize>>();
        players.shuffle(&mut thread_rng());
        players.pop().unwrap()
    }

    pub fn random(player_count: u8) -> Game {
        Game {
            player_count,
            board: Board::random(),
            hands: [0; 4],
            devs: [0; 4],
            played_knights: [0; 4],
            largest_army_player: 0,
            longest_road_player: 0,
            max_cards: if player_count == 2 { 9 } else { 7 },
            orbit: 0,
        }
    }
    
}