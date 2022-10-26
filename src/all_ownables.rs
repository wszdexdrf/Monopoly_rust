#[derive(Debug)]
pub struct Property {
    pub price: f64,
    pub color: u8,
    buildings: i32,
    rents: [f64; 6],
    is_mortgaged: bool,
    pub monopoly: bool,
}
impl Property {
    pub fn new(price: f64, color: u8, rents: [f64; 6]) -> Self {
        Self {
            price,
            buildings: 0,
            rents,
            is_mortgaged: false,
            color,
            monopoly: false,
        }
    }
    pub fn reset(&mut self) {
        self.buildings = 0;
        self.is_mortgaged = false;
        self.monopoly = false;
    }
    pub fn get_rent(&self) -> f64 {
        if self.is_mortgaged {
            0.0
        } else if self.buildings == 0 && self.monopoly {
            self.rents[0] * 2.0
        } else {
            self.rents[self.buildings as usize]
        }
    }
    pub fn mortgage(&mut self) -> f64 {
        if !self.is_mortgaged {
            self.is_mortgaged = true;
            let temp = self.buildings;
            self.buildings = 0;
            return self.price / 2.0 + (temp * (self.color as i32 / 2 + 1)) as f64 * 50.0;
        } else {
            return 0.0;
        }
    }
    pub fn unmortgage(&mut self) -> f64 {
        if self.is_mortgaged {
            self.is_mortgaged = false;
            return self.price / 2.0 * 1.1;
        } else {
            return 0.0;
        }
    }
    pub fn is_mortgaged(&self) -> bool {
        self.is_mortgaged
    }

    pub fn make_building(&mut self, num_buildings: i32) -> f64 {
        let house_price: f64 = (self.color as i32 / 2 * 50 + 50) as f64;
        if self.monopoly
            && self.buildings + num_buildings < 6
            && self.buildings + num_buildings >= 0
            && !self.is_mortgaged
        {
            self.buildings += num_buildings;
            if num_buildings > 0 {
                num_buildings as f64 * house_price
            } else {
                -(num_buildings as f64 * house_price / 2.0)
            }
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct Railroad {
    pub price: f64,
    pub color: u8,
    rent: f64,
    multiplier: f64,
    is_mortgaged: bool,
}
impl Railroad {
    pub fn new() -> Self {
        Self {
            price: 200.0,
            color: b'r',
            rent: 25.0,
            multiplier: 1.0,
            is_mortgaged: false,
        }
    }
    pub fn reset(&mut self) {
        self.multiplier = 1.0;
        self.is_mortgaged = false;
    }
    pub fn get_rent(&self) -> f64 {
        if self.is_mortgaged {
            0.0
        } else {
            self.rent * self.multiplier
        }
    }
    pub fn add_one(&mut self) {
        self.multiplier *= 2.0;
    }
    pub fn mortgage(&mut self) -> f64 {
        if !self.is_mortgaged {
            self.is_mortgaged = true;
            return self.price / 2.0;
        } else {
            return 0.0;
        }
    }
    pub fn unmortgage(&mut self) -> f64 {
        if self.is_mortgaged {
            self.is_mortgaged = false;
            return self.price / 2.0 * 1.1;
        } else {
            return 0.0;
        }
    }
    pub fn is_mortgaged(&self) -> bool {
        self.is_mortgaged
    }
}

#[derive(Debug)]
pub struct Utility {
    pub price: f64,
    pub color: u8,
    is_mortgaged: bool,
    pub monopoly: bool,
}
impl Utility {
    pub fn new() -> Self {
        Self {
            price: 150.0,
            color: b'u',
            is_mortgaged: false,
            monopoly: false,
        }
    }

    pub fn reset(&mut self) {
        self.is_mortgaged = false;
        self.monopoly = false;
    }

    pub fn get_rent(&self, moves: usize) -> f64 {
        if self.is_mortgaged {
            0.0
        } else {
            moves as f64 * if self.monopoly { 10.0 } else { 4.0 }
        }
    }
    pub fn mortgage(&mut self) -> f64 {
        if !self.is_mortgaged {
            self.is_mortgaged = true;
            return self.price / 2.0;
        } else {
            return 0.0;
        }
    }
    pub fn unmortgage(&mut self) -> f64 {
        if self.is_mortgaged {
            self.is_mortgaged = false;
            return self.price / 2.0 * 1.1;
        } else {
            return 0.0;
        }
    }

    pub fn is_mortgaged(&self) -> bool {
        self.is_mortgaged
    }
}
