use all_ownables::{Property, Railroad, Utility};
use ownable::Ownable;
use player::Player;
use std::{thread, time::Instant};

mod all_ownables;
mod basic_agent;
mod ownable;
mod player;

fn main() {
    let start_time = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..10_000 {
        handles.push(thread::spawn(|| {
            let rents = [
                [2.0, 10.0, 30.0, 90.0, 160.0, 250.0],
                [4.0, 20.0, 60.0, 180.0, 320.0, 450.0],
                [6.0, 30.0, 90.0, 270.0, 400.0, 550.0],
                [8.0, 40.0, 100.0, 300.0, 450.0, 600.0],
                [10.0, 50.0, 150.0, 450.0, 625.0, 750.0],
                [12.0, 60.0, 180.0, 500.0, 700.0, 900.0],
                [14.0, 70.0, 200.0, 550.0, 750.0, 950.0],
                [16.0, 80.0, 220.0, 600.0, 800.0, 1000.0],
                [18.0, 90.0, 250.0, 700.0, 875.0, 1050.0],
                [20.0, 100.0, 300.0, 750.0, 925.0, 1100.0],
                [22.0, 110.0, 330.0, 800.0, 975.0, 1150.0],
                [24.0, 120.0, 360.0, 850.0, 1025.0, 1200.0],
                [26.0, 130.0, 390.0, 900.0, 1100.0, 1275.0],
                [28.0, 150.0, 450.0, 1000.0, 1200.0, 1400.0],
                [35.0, 175.0, 500.0, 1100.0, 1300.0, 1500.0],
                [50.0, 200.0, 600.0, 1400.0, 1700.0, 2000.0],
            ];
            Game::create_instance(4, &rents).play();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // let rents = [
    //     [2.0, 10.0, 30.0, 90.0, 160.0, 250.0],
    //     [4.0, 20.0, 60.0, 180.0, 320.0, 450.0],
    //     [6.0, 30.0, 90.0, 270.0, 400.0, 550.0],
    //     [8.0, 40.0, 100.0, 300.0, 450.0, 600.0],
    //     [10.0, 50.0, 150.0, 450.0, 625.0, 750.0],
    //     [12.0, 60.0, 180.0, 500.0, 700.0, 900.0],
    //     [14.0, 70.0, 200.0, 550.0, 750.0, 950.0],
    //     [16.0, 80.0, 220.0, 600.0, 800.0, 1000.0],
    //     [18.0, 90.0, 250.0, 700.0, 875.0, 1050.0],
    //     [20.0, 100.0, 300.0, 750.0, 925.0, 1100.0],
    //     [22.0, 110.0, 330.0, 800.0, 975.0, 1150.0],
    //     [24.0, 120.0, 360.0, 850.0, 1025.0, 1200.0],
    //     [26.0, 130.0, 390.0, 900.0, 1100.0, 1275.0],
    //     [28.0, 150.0, 450.0, 1000.0, 1200.0, 1400.0],
    //     [35.0, 175.0, 500.0, 1100.0, 1300.0, 1500.0],
    //     [50.0, 200.0, 600.0, 1400.0, 1700.0, 2000.0],
    // ];
    // Game::create_instance(4, &rents).play();
    let end_time = Instant::now();
    println!(
        "\n{} seconds",
        end_time.duration_since(start_time).as_secs_f64() / 10_000.0
    );
}

struct Game {
    map: Vec<Ownable>,
    players: Vec<Player>,
    eliminated_players: Vec<bool>,
    ownable_ids: Vec<Option<(u8, usize)>>,
}

impl Game {
    pub fn create_instance(num_players: usize, rents: &[[f64; 6]; 16]) -> Self {
        let mut current = Self {
            map: Vec::with_capacity(40),
            players: (0..num_players).map(|_| Player::new(1500.0)).collect(),
            eliminated_players: vec![false; num_players],
            ownable_ids: Vec::with_capacity(40),
        };
        (0..40).for_each(|_| current.map.push(Ownable::N()));
        (0..40).for_each(|i| current.ownable_ids.push(current.get_property_id(i)));
        (0..5).for_each(|c: usize| {
            current.create_properties(
                &[c * 5 + 1, c * 5 + 3, c * 5 + 4],
                c as u8,
                60.0 + c as f64 * 40.0,
                80.0 + c as f64 * 40.0,
                if c == 0 { 2 } else { 3 },
                &rents[c * 2],
                &rents[c * 2 + 1],
            )
        });

        (5..7).for_each(|c: usize| {
            current.create_properties(
                &[c * 5 + 1, c * 5 + 2, c * 5 + 4],
                c as u8,
                60.0 + c as f64 * 40.0,
                80.0 + c as f64 * 40.0,
                if c == 0 { 2 } else { 3 },
                &rents[c * 2],
                &rents[c * 2 + 1],
            )
        });

        current.create_properties(&[37, 39], 7, 350.0, 400.0, 2, &rents[14], &rents[15]);

        (0..4).for_each(|index: usize| current.map[index * 10 + 5] = Ownable::R(Railroad::new()));

        current.map[12] = Ownable::U(Utility::new());
        current.map[28] = Ownable::U(Utility::new());

        current.map[4] = Ownable::T(200.0);
        current.map[38] = Ownable::T(100.0);

        current
    }

    fn create_properties(
        &mut self,
        locations: &[usize],
        color: u8,
        l_price: f64,
        h_price: f64,
        num_prop: usize,
        l_rents: &[f64; 6],
        h_rents: &[f64; 6],
    ) {
        if color < 7 {
            for i in 0..num_prop - 1 {
                self.map[locations[i]] =
                    Ownable::P(Property::new(l_price, color, Clone::clone(l_rents)));
            }
            self.map[locations[num_prop - 1]] =
                Ownable::P(Property::new(h_price, color, Clone::clone(h_rents)));
        } else if color == 7 {
            self.map[locations[0]] = Ownable::P(Property::new(l_price, 7, Clone::clone(l_rents)));
            self.map[locations[1]] = Ownable::P(Property::new(h_price, 7, Clone::clone(h_rents)));
        }
    }

    fn roll_die() -> usize {
        rand::random::<usize>() % 6 + rand::random::<usize>() % 6 + 2
    }

    fn move_player(plyr: &mut Player, moves: usize) -> usize {
        plyr.location += moves;
        if plyr.location >= 40 {
            plyr.location -= 40;
            plyr.take_money(200.0);
        }
        plyr.location
    }

    fn get_property_id(&self, location: usize) -> Option<(u8, usize)> {
        // CAN BE CACHED INTO AN ARRAY (Already done) ------------------ !
        let tier = location as u8 / 5;
        let position = location % 5;
        if tier < 5 {
            if tier == 0 && position == 4 {
                return None;
            } else if position != 0 && position != 2 {
                return Some((tier, if position == 1 { 0 } else { position - 2 }));
            } else if position == 0 {
                if tier % 2 == 0 {
                    return Some((b'r', tier as usize / 2));
                } else {
                    return None;
                }
            } else {
                if tier == 2 {
                    return Some((b'u', 0));
                } else {
                    return None;
                }
            }
        } else {
            if tier == 7 && position == 1 {
                return None;
            } else if position != 0 && position != 3 {
                return Some((
                    tier,
                    if position == 4 {
                        if tier == 7 {
                            1
                        } else {
                            2
                        }
                    } else {
                        if tier == 7 {
                            0
                        } else {
                            position - 1
                        }
                    },
                ));
            } else if position == 0 {
                if tier % 2 == 0 {
                    return Some((b'r', tier as usize / 2));
                } else {
                    return None;
                }
            } else {
                if tier == 5 {
                    return Some((b'u', 1));
                } else {
                    return None;
                }
            }
        }
    }

    pub fn take_ownable(&mut self, mut ownable_obj: Ownable, tier: u8, position: usize) {
        match &mut ownable_obj {
            Ownable::P(s) => s.reset(),
            Ownable::R(s) => s.reset(),
            Ownable::U(s) => s.reset(),
            _ => (),
        }
        if tier == b'u' {
            self.map[if position == 0 { 12 } else { 28 }] = ownable_obj;
        } else if tier == b'r' {
            self.map[position * 10 + 5] = ownable_obj;
        } else if tier < 5 {
            self.map[tier as usize * 5 + [1, 3, 4][position]] = ownable_obj;
        } else if tier < 7 {
            self.map[tier as usize * 5 + [1, 2, 4][position]] = ownable_obj;
        } else {
            self.map[[37, 39][position]] = ownable_obj;
        }
    }

    pub fn play(&mut self) {
        for _ in 0..100 {
            for player_index in 0..self.players.len() {
                if self.eliminated_players[player_index] {
                    continue;
                }
                // Roll Die
                let die = Game::roll_die();
                // Get new location
                let location = Game::move_player(&mut self.players[player_index], die);
                let mut rent = 0.0;
                let mut purchaseable = true;
                let (mut tr, mut pos) = (0, 0);
                let mut owner = None;
                // Check about the location
                match self.ownable_ids[location] {
                    // if it is a Ownable item
                    Some(t) => {
                        tr = t.0;
                        pos = t.1;
                        // Check which player owns it. Get rent.
                        for plr in 0..self.players.len() {
                            match self.players[plr].get_rent(t.1, t.0, die) {
                                Some(r) => {
                                    rent = if plr != player_index { r } else { 0.0 };
                                    purchaseable = false;
                                    owner = Some(plr);
                                    break;
                                }
                                None => (),
                            }
                        }
                    }
                    // May be tax position or nothing (Chance, free parking etc.)
                    None => {
                        purchaseable = false;
                        rent = if let Ownable::T(r) = self.map[location] {
                            r
                        } else {
                            0.0
                        };
                    }
                }
                // If item is purchasable
                if purchaseable {
                    // place None placeholder to replace in bank's owned items
                    self.map.push(Ownable::N());
                    if let Some(d) = basic_agent::buy_or_not(
                        // if purchased
                        &mut self.players[player_index],
                        self.map.swap_remove(location),
                        tr,
                        pos,
                    ) {
                        self.map[location] = d;
                    }
                } else {
                    // else pay rent / tax
                    let mut paid = self.players[player_index].pay_money(rent);
                    let mut package: Vec<(Ownable, u8, usize)> = Vec::new();
                    // if paid == -1 then player can pay if he/she mortgages properties.
                    while paid == -1.0 {
                        basic_agent::raise_money_somehow(&mut self.players[player_index], rent);
                        paid = self.players[player_index].pay_money(rent);
                    }
                    // if paid is < rent but not -1 then player gave everything he had.
                    if paid != rent {
                        self.eliminated_players[player_index] = true;
                        self.players.push(Player::new(0.0));
                        package = self.players.swap_remove(player_index).return_ownables();
                    }
                    // if owner was another player
                    if let Some(index) = owner {
                        self.players[index].take_money(paid);
                        // if current player was eliminated
                        if self.eliminated_players[player_index] {
                            // take ownership of everything in package
                            package
                                .into_iter()
                                .filter(
                                    |(obj, _, _)| if let Ownable::N() = obj { false } else { true },
                                )
                                .for_each(|(obj, t, pos)| {
                                    self.players[index].take_ownable(obj, t, pos)
                                });
                        }
                    } else {
                        // if owner is not a player i.e. bank

                        // if player is eliminated
                        if self.eliminated_players[player_index] {
                            // bank takes ownership of everything in package
                            package
                                .into_iter()
                                .filter(
                                    |(obj, _, _)| if let Ownable::N() = obj { false } else { true },
                                )
                                .for_each(|(obj, t, pos)| self.take_ownable(obj, t, pos));
                        }
                    }
                }
                // print the state of current player
                // println!(
                //     "Player {} at {} with $ {} Net {} ",
                //     player_index,
                //     self.players[player_index].location,
                //     self.players[player_index].money,
                //     self.players[player_index].net_worth
                // );

                // if number of un-eliminated players is 1
                // if self
                //     .eliminated_players
                //     .iter()
                //     .map(|&v| if v { 0 } else { 1 })
                //     .sum::<i32>()
                //     == 1
                // {
                //     println!(
                //         "Player {} wins !!!",
                //         self.eliminated_players.iter().position(|&b| !b).unwrap()
                //     );
                //     return;
                // }
                // BUILD OR NOT
                basic_agent::build_or_not(&mut self.players[player_index]);

                // TRADE OR NOT ------------------- !
                // UNMORTGAGE OR NOT ------------------- !
            }
            // println!();
        }
    }
}
