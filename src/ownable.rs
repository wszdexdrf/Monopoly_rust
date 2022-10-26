use crate::all_ownables::{Property, Railroad, Utility};

#[derive(Debug)]
pub enum Ownable {
    P(Property),
    R(Railroad),
    U(Utility),
    T(f64),
    N(),
}

pub fn get_size_of_tier(tier: u8) -> usize {
    if tier == 0 || tier == 7 || tier == b'u' {
        2
    } else if tier == b'r' {
        4
    } else {
        3
    }
}
