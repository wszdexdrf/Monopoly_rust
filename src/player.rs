use std::collections::HashMap;

use crate::ownable::{get_size_of_tier, Ownable};

pub struct Player {
    pub location: usize,
    pub money: f64,
    pub net_worth: f64,
    owned: HashMap<u8, Vec<Ownable>>,
}
impl Player {
    pub fn new(starting_money: f64) -> Self {
        Self {
            location: 0,
            money: starting_money,
            owned: HashMap::new(),
            net_worth: starting_money,
        }
    }

    pub fn get_rent(&self, ownable_index: usize, key: u8, moves: usize) -> Option<f64> {
        match self.owned.get(&key) {
            Some(v) => match &v[ownable_index] {
                Ownable::P(ownable_obj) => Some(ownable_obj.get_rent()),
                Ownable::R(ownable_obj) => Some(ownable_obj.get_rent()),
                Ownable::U(ownable_obj) => Some(ownable_obj.get_rent(moves)),
                _ => None,
            },
            None => None,
        }
    }

    pub fn mortgage(&mut self, ownable_index: usize, key: u8) {
        match self.owned.get_mut(&key) {
            Some(v) => match &mut v[ownable_index] {
                Ownable::P(ownable_obj) => self.money += ownable_obj.mortgage(),
                Ownable::R(ownable_obj) => self.money += ownable_obj.mortgage(),
                Ownable::U(ownable_obj) => self.money += ownable_obj.mortgage(),
                _ => panic!("Incorrect Ownable is here"),
            },
            None => panic!("Ownable is not owned"),
        }
    }

    pub fn pay_money(&mut self, due: f64) -> f64 {
        if self.money >= due {
            self.money -= due;
            self.net_worth -= due;
            return due;
        } else if self.net_worth >= due {
            return -1.0;
        } else {
            return self.money;
        }
    }

    pub fn take_money(&mut self, money: f64) {
        self.money += money;
        self.net_worth += money;
    }

    pub fn take_ownable(&mut self, ownable_obj: Ownable, tier: u8, position: usize) {
        match &ownable_obj {
            Ownable::P(obj) => {
                if !(obj.is_mortgaged()) {
                    self.net_worth += obj.price / 2.0;
                }
            }
            Ownable::R(obj) => {
                if !(obj.is_mortgaged()) {
                    self.net_worth += obj.price / 2.0;
                }
            }
            Ownable::U(obj) => {
                if !(obj.is_mortgaged()) {
                    self.net_worth += obj.price / 2.0;
                }
            }
            _ => (),
        };
        match self.owned.get_mut(&tier) {
            Some(v) => {
                v[position] = ownable_obj;
                if tier == b'u' {
                    v.iter_mut().for_each(|obj| {
                        if let Ownable::U(util) = obj {
                            util.monopoly = true;
                        }
                    });
                } else if tier == b'r' {
                    v.iter_mut().for_each(|obj| {
                        if let Ownable::R(rail) = obj {
                            rail.add_one();
                        }
                    });
                } else {
                    let mut count = 0;
                    v.iter_mut()
                        .filter(|obj| if let Ownable::N() = obj { false } else { true })
                        .for_each(|_| count += 1);
                    if count == get_size_of_tier(tier) {
                        v.iter_mut().for_each(|obj| {
                            if let Ownable::P(prop) = obj {
                                prop.monopoly = true;
                            }
                        });
                    }
                }
            }
            None => {
                // v = vec![Ownable::N(); get_size_of_tier(tier)];
                let tier_size = get_size_of_tier(tier);
                let mut v = Vec::with_capacity(tier_size);
                (0..tier_size).for_each(|_| v.push(Ownable::N()));
                v[position] = ownable_obj;
                self.owned.insert(tier, v);
            }
        }
    }

    pub fn return_ownables(self) -> Vec<(Ownable, u8, usize)> {
        let mut package = Vec::new();
        self.owned.into_iter().for_each(|(k, v)| {
            v.into_iter()
                .enumerate()
                .for_each(|(position, object)| package.push((object, k, position)))
        });
        package
    }

    pub fn check_monopoly(&mut self, tier: u8) -> bool {
        let size_tier = get_size_of_tier(tier);
        if let Some(v) = self.owned.get(&tier) {
            v.len() == size_tier
        } else {
            false
        }
    }

    pub fn make_building(&mut self, num_buildings: i32, tier: u8, index: usize) {
        if let Some(v) = self.owned.get_mut(&tier) {
            if let Ownable::P(ownable_obj) = &mut v[index] {
                let cost = ownable_obj.make_building(num_buildings);
                if self.money >= cost
                    && !v.iter().any(|ownble| {
                        if let Ownable::P(obj) = ownble {
                            obj.is_mortgaged()
                        } else {
                            false
                        }
                    })
                {
                    self.money -= cost;
                    self.net_worth -= cost / 2.0;
                }
            }
        }
    }

    pub fn destroy_building(&mut self, num_buildings: i32, tier: u8, index: usize) {
        if let Some(v) = self.owned.get_mut(&tier) {
            if let Ownable::P(ownable_obj) = &mut v[index] {
                let cost = ownable_obj.make_building(-num_buildings);
                self.money += cost;
            }
        }
    }

    pub fn is_mortgaged(&self, tier: u8, index: usize) -> bool {
        if let Some(v) = self.owned.get(&tier) {
            match &v[index] {
                Ownable::P(obj) => obj.is_mortgaged(),
                Ownable::R(obj) => obj.is_mortgaged(),
                Ownable::U(obj) => obj.is_mortgaged(),
                _ => false,
            }
        } else {
            false
        }
    }
}
