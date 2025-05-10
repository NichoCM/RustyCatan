use std::{u8, vec};

use rand::rngs::ThreadRng;

use crate::board::{self, building::{Building, BUILDING_SHIFT, PLAYER_VALUE_MASK}, coordinate::{Coordinate2, Coordinate3}, tile::{Tile, TILE_SHIFT, TILE_VALUE_MASK}, Board};

use super::{hand::{self, MAX_CARDS}, Game};

#[derive(Debug)]
pub enum PlayerAction {
    FirstPlacement,
    FirstSettlement(Coordinate3),
    InitialRoadPlacement(Coordinate3),
    SecondPlacement,
    SecondSettlement(Coordinate3),
    Discard,
    PlaceSettlement(Coordinate3),
    PlaceCity(Coordinate3), 
    PlaceRoad(Coordinate3),
    RobberMove(Coordinate2),
    RobberSteal(u8, u8),
    Port(i8, u8, u8),
    BuyDev,
    PlayKnight,
    PlayYOP,
    PlayMono,
    PlayRoad,
    Pass,
}

#[derive(PartialEq, Debug)]
pub enum GameAction {
    GameStart,
    FirstPlacement(u8), // Provide the player
    SecondPlacement(u8), // Provide the player
    PlacementFinished, // Action just to indicate all the placements are done
    Roll(u8, u8),
    TurnDone,
    Finished,
}

impl Game {

    /**
     * Perform a player action, and return a list of possible actions. Returning an
     * empty vector means there are no more actions and the game should move to the next player
     * 
     * Returns 2 values, the vector of actions and a boolean flag indicating if a developmentg card was
     */
    pub fn perform_player_action(&mut self, rng: &mut ThreadRng, player: u8, action: PlayerAction) -> Vec<PlayerAction> {
        println!("Player {} {:?}", player, action);
        let mut vec = Vec::new();
        match action {
            PlayerAction::FirstPlacement => {
                // Get the valid first placement spots
                self.board.get_valid_starting_buiding_coords().into_iter().for_each(|e| {
                    vec.push(PlayerAction::FirstSettlement(e));
                });
            }
            PlayerAction::FirstSettlement(coord) => {
                // Set the settlement
                self.board.set_building(&coord, Building::Settlement, player);

                // Get the available road placements
                self.board.get_placement_roads_around(&coord).into_iter().for_each(|e| {
                    vec.push(PlayerAction::InitialRoadPlacement(e));
                });
            }
            PlayerAction::SecondPlacement => {
                // Get the valid first placement spots
                self.board.get_valid_starting_buiding_coords().into_iter().for_each(|e| {
                    vec.push(PlayerAction::SecondSettlement(e));
                });
            }
            PlayerAction::SecondSettlement(coord) => {
                // Set the settlement
                self.board.set_building(&coord, Building::Settlement, player);

                // Get the available road placements
                self.board.get_placement_roads_around(&coord).into_iter().for_each(|e| {
                    vec.push(PlayerAction:: InitialRoadPlacement(e));
                });
                self.distribute_initial_resources(player, &coord);
            }
            PlayerAction::InitialRoadPlacement(coord) => {
                // Set the road
                self.board.set_road(&coord, player);
            }
            PlayerAction::Discard => {
                let old_hand = self.get_hand(player);
                let new_hand = self.call_player_discard(player);
                let old_size = hand::size(old_hand);
                if old_size - hand::size(new_hand) == old_size / 2 {
                    self.set_hand(player, new_hand);
                } else {
                    // Invalid discard, this can either force a forfeit or require them to pick again.
                    // For now, get the agent to pick again
                    vec.push(PlayerAction::Discard);
                }
            }
            PlayerAction::RobberMove(coord) => {

                let mut has_players = false;
                let mut can_steal: [bool; 4] = [false; 4];

                // Get robabble players adjacent to these coordinates
                Board::get_tile_adjacent_vertex_coords(&coord).into_iter().for_each(|e| {
                    let p = self.board.get_building(&e) & PLAYER_VALUE_MASK;
                    if p != 0 && player != p {
                        let hand = self.get_hand(p);
                        if hand::size(hand) != 0 {
                            has_players = true;
                            can_steal[(p - 1) as usize] = true;
                        }
                    }
                });

                // If there are robbable players, ask the player agent which player they should steal
                if has_players {
                    let steal= self.call_player_rob(can_steal);
                    if can_steal[steal] {
                        let other_player = (steal + 1) as u8;
                        let hand = self.get_hand(other_player);
                        let res = Game::steal(hand, rng);
                        vec.push(PlayerAction::RobberSteal(other_player, res));
                    } else {
                        // Either automatically resign the player, or request the the steal again
                        // For now, we'll just re-request the robber move
                        return vec![PlayerAction::RobberMove(coord)];
                    }
                }

                // Set the robber position
                self.board.robber = coord;
            }
            PlayerAction::RobberSteal(other_player, res) => {
                // Remove the resource from the target player
                let hand = self.get_hand(other_player);
                let hand = hand::add(hand, res, -1);
                self.set_hand(other_player, hand);

                // Update the current player's hand with the stolen resource
                let hand = self.get_hand(player);
                let hand = hand::add(hand, res, 1);
                self.set_hand(player, hand);

                // Get more player actions
                vec.append(&mut self.get_turn_actions(player));
            }
            PlayerAction::PlaceRoad(coord) => {
                // Subtract the Road from the player's hand
                let hand = self.get_hand(player);
                let hand = hand::subtract(hand, hand::ROAD_RECIPE);
                self.set_hand(player, hand);

                // Set the road
                self.board.set_road(&coord, player);

                // Get more player actions
                vec.append(&mut self.get_turn_actions(player));
            }
            PlayerAction::PlaceSettlement(coord) => {
                let hand = self.get_hand(player);
                let hand = hand::subtract(hand, hand::SETTLE_RECIPE);
                self.set_hand(player, hand);

                // Set the settlement
                self.board.set_building(&coord, Building::Settlement, player);
  
                // Get more player actions
                vec.append(&mut self.get_turn_actions(player));
            }
            PlayerAction::PlaceCity(coord) => {
                let hand = self.get_hand(player);
                let hand = hand::subtract(hand, hand::CITY_RECIPE);
                self.set_hand(player, hand);

                // Set the settlement
                self.board.set_building(&coord, Building::City, player);
  
                // Get more player actions
                vec.append(&mut self.get_turn_actions(player));
            }
            PlayerAction::PlayKnight => {
                // Move the robber
                self.get_robbable_tiles().into_iter().for_each(|e| {
                    vec.push(PlayerAction::RobberMove(e))
                });
            }
            PlayerAction::Port(amount, from, to) => {
                // Take 4 of the from resources and turn it into 1 of the to resources
                let hand = self.get_hand(player);
                let hand = hand::add(hand, from, -amount);
                let hand = hand::add(hand, to, 1);
                self.set_hand(player, hand);
                vec.append(&mut self.get_turn_actions(player));
            },
            _=> {}
        }

        // Check if a player has won. If so, go back to the game 
        if self.has_winner() {
            return Vec::new();
        }

        vec
    }

    /**
     * Perform a game action and return the next action. Game actions require no decisions,
     * and hence only one must be returned.
     */
    pub fn perform_game_action(&mut self, rng: &mut ThreadRng, action: GameAction) -> GameAction {
        match action {
            GameAction::GameStart => GameAction::FirstPlacement(1),
            GameAction::FirstPlacement(player) => {
                let actions = self.perform_player_action(rng, player, PlayerAction::FirstPlacement);
                self.call_player(rng, player, actions);
                if player == self.player_count {
                    GameAction::SecondPlacement(player)
                } else {
                    GameAction::FirstPlacement(player + 1)
                }
            }
            GameAction::SecondPlacement(player) => {
                let actions = self.perform_player_action(rng, player, PlayerAction::SecondPlacement);
                self.call_player(rng, player, actions);
                if player == 1 {
                    GameAction::PlacementFinished
                } else {
                    GameAction::SecondPlacement(player - 1)
                }
            }
            GameAction::Roll(player, value) => {
                println!("Roll {} : {}", value, player);
                if value == 7 {
                    
                    // Check if a player must discard cards
                    for p in 1 ..= self.player_count {
                        let hand = self.get_hand(p);
                        if hand::size(hand) > self.max_cards.into() {
                            self.perform_player_action(rng, p, PlayerAction::Discard);
                        }
                    }
            
                    // Possible robber action for current player
                    let actions = self.get_robbable_tiles().into_iter().map(|e| {
                        PlayerAction::RobberMove(e)
                    }).collect();
                    self.call_player(rng, player, actions);
                } else {
                    self.distribute_resources_for_roll(value);
                }

                let actions = self.get_turn_actions(player);
                self.call_player(rng, player, actions);
            
                GameAction::TurnDone
            }
            _ => GameAction::Finished,
        }
    }

    /**
     * Handle distributing the inital resource around a settlement at the specific vertex
     * coordinate.
     */
    fn distribute_initial_resources(&mut self, player: u8, coord: &Coordinate3) {
        let robber: crate::board::coordinate::Coordinate2 = self.board.robber.clone();
        Board::get_vertex_adjacent_tile_coords(coord).into_iter().filter(|e|
            e.is_valid_tile() && robber != *e
        ).for_each(|e| {
            let (tile, _) = Tile::from_u8(self.board.get_tile(&e));
            let hand = self.get_hand(player);
            let new_hand = match tile {
                Tile::Brick => {
                    hand::add_brick(hand, 1)
                },
                Tile::Lumber => {
                    hand::add_lumber(hand, 1)
                },
                Tile::Sheep => {
                    hand::add_sheep(hand, 1)
                },
                Tile::Ore => {
                    hand::add_ore(hand, 1)
                },
                Tile::Wheat => {
                    hand::add_wheat(hand, 1)
                },
                _ => hand,
            };
            self.set_hand(player, new_hand);
        });
    }

    /**
     * Handle distributing resources after a roll
     */
    fn distribute_resources_for_roll(&mut self, value: u8) {
        let mut ops: Vec<(u8, u8, u8)> = Vec::new();
        let mut totals = [
            self.get_resource_card_count(1),
            self.get_resource_card_count(2),
            self.get_resource_card_count(3),
            self.get_resource_card_count(4),
            self.get_resource_card_count(5),
        ];

        // Generate a list of possible operations. This allows us to make sure
        // there are enough resources to distribute.
        Board::get_tile_coords().into_iter().for_each(|e| {
            let tile= self.board.get_tile(&e);
            if e != self.board.robber && tile & TILE_VALUE_MASK == value {
                // Get the numeric Tile ID
                let tile = tile >> TILE_SHIFT;
                for v in Board::get_tile_adjacent_vertex_coords(&e) {
                    let building = self.board.get_building(&v);
                    if building != 0 {
                        let player = building & PLAYER_VALUE_MASK;
                        let building = building >> BUILDING_SHIFT;

                        // Update the totals in the building
                        totals[(tile - 1) as usize] += building as u32;
                        ops.push((player, tile, building));
                    }
                }
            }
        });

        // Apply the operations if there are enough resources to do it
        ops.into_iter().for_each(|(player, tile, building)| {
            if totals[(tile - 1) as usize] <= MAX_CARDS {
                let hand = self.get_hand(player);
                let hand = hand::add(hand, tile, building as i8);
                self.set_hand(player, hand);
            }
        });
    }

    /**
     * Get the sum of all resource cards by a specific type 
     * which are currently in player hands
     */
    fn get_resource_card_count(&self, res: u8) -> u32 {
        let mut sum: u32 = 0;
        for p in 0 ..= 3 {
            let hand = self.hands[p];
            sum += hand::count(hand, res);
        }
        sum
    }

    /**
     * Get all the robbable tiles (excludes the current tile the robber is on)
     */
    fn get_robbable_tiles(&self) -> Vec<Coordinate2> {
        Board::get_tile_coords().into_iter().filter(|e| {
            &self.board.robber != e
        }).collect()
    }

    /**
     * Helper function to generate all the legal turn actions
     */
    pub fn get_turn_actions(&self, player: u8) -> Vec<PlayerAction> {
        // TODO - add dev support, only one dev card can be player a turn
        let mut vec = Vec::new();

        let hand = self.get_hand(player);

        // Road Building
        if hand::has(hand, hand::ROAD_RECIPE) {
            println!("{:?}", self.get_legal_road_locations(player).len());
            self.get_legal_road_locations(player).into_iter().for_each(|e| {
                vec.push(PlayerAction::PlaceRoad(e))
            });
        }

        // Settlement
        if hand::has(hand, hand::SETTLE_RECIPE) {
            self.get_legal_settle_locations(player).into_iter().for_each(|e| {
                vec.push(PlayerAction::PlaceSettlement(e));
            });
        }

        // City
        if hand::has(hand, hand::CITY_RECIPE) {
            self.get_legal_city_locations(player).into_iter().for_each(|e| {
                vec.push(PlayerAction::PlaceCity(e));
            });
        }

        // Ports
        vec.append(&mut self.get_port_actions(player));

        // Pass
        vec.push(PlayerAction::Pass);
        
        vec
    }

    /**
     * Get all the actions
     */
    fn get_port_actions(&self, player: u8) -> Vec<PlayerAction> {
        let mut vec = Vec::new();
        let mut ports = [false; 6];
        board::PORT_LOCATIONS.into_iter().enumerate().for_each(|(i, (x, y, z))| {
            Board::get_edge_adjacent_vertex_coords(&Coordinate3::new(x, y, z))
                .into_iter()
                .for_each(|e| {
                    if self.board.get_building(&e) & PLAYER_VALUE_MASK == player {
                        ports[self.board.ports[i] as usize] = true;
                    }
                });
        });

        let hand: u32 = self.get_hand(player);
        for res in 1 ..= 5 {
            let count = hand::count(hand, res);
            if count >= 4 {
                for to in 1 ..= 5 {
                    if res != to {
                        if ports[res as usize] {
                            if count >= 2 {
                                vec.push(PlayerAction::Port(2, res, to));
                            }
                        } else if ports[0] {
                            if count >= 3 {
                                vec.push(PlayerAction::Port(3, res, to));
                            }
                        } else {
                            if count >= 4 {
                                vec.push(PlayerAction::Port(4, res, to));
                            }
                        }
                    }
                }
            }
        }

        vec
    }

    /**
     * Get the locations that a player can build
     */
    fn get_legal_settle_locations(&self, player: u8) -> Vec<Coordinate3> {
        // Get the road locations that belong to the 
        self.board.get_valid_starting_buiding_coords().into_iter().filter(|e| {
            // Make sure the player has a road adjacent to this vertex
            for e in Board::get_vertex_adjacent_edge_coords(e) {
                let road = self.board.get_road(e.x, &Coordinate2::new(e.y, e.z));
                if road == player {
                    return true
                }
            }
            false
        }).collect()
    }

    /**
     * Get the locations that a player can build
     */
    fn get_legal_road_locations(&self, player: u8) -> Vec<Coordinate3> {
        // Get the road locations that belong to the 
        Board::get_edge_coords().into_iter().filter(|e| {
            let road = self.board.get_road(e.x, &Coordinate2::new(e.y, e.z));

            // If this road is already built, you can't build here
            if road != 0 {
                return false
            }

            // If not, check to see if any of the adjacent roads is the player's
            for v in Board::get_edge_adjacent_vertex_coords(e) {
                for e in Board::get_vertex_adjacent_edge_coords(&v) {
                    // Check to see if any of the adjacent roads are available
                    let road = self.board.get_road(e.x, &Coordinate2::new(e.y, e.z));
                    if road == player {
                        return true
                    }
                }
            }

            false
        }).collect()
    }

    /**
     * Get the legal city locations for a player
     */
    fn get_legal_city_locations(&self, player: u8) -> Vec<Coordinate3> {
        Board::get_vertex_coords().into_iter().filter(|e| {
            let building = self.board.get_building(e);
            building & PLAYER_VALUE_MASK == player && (building >> BUILDING_SHIFT) == 1
        }).collect()
    }

}