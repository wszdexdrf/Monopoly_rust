use crate::{
    ownable::{get_size_of_tier, Ownable},
    player::Player,
};

pub fn buy_or_not(player: &mut Player, ownable: Ownable, tier: u8, pos: usize) -> Option<Ownable> {
    match &ownable {
        Ownable::P(s) => {
            if s.price < player.money {
                player.pay_money(s.price);
                player.take_ownable(ownable, tier, pos);
                return None;
            }
        }
        Ownable::R(s) => {
            if s.price < player.money {
                player.pay_money(s.price);
                player.take_ownable(ownable, tier, pos);
                return None;
            }
        }
        Ownable::U(s) => {
            if s.price < player.money {
                player.pay_money(s.price);
                player.take_ownable(ownable, tier, pos);
                return None;
            }
        }
        _ => (),
    }
    return Some(ownable);
}

pub fn raise_money_somehow(player_obj: &mut Player, due: f64) {
    for t in 0..8 {
        if player_obj.check_monopoly(t) {
            for p in 0..get_size_of_tier(t) {
                player_obj.destroy_building(1, t, p);
                if player_obj.money >= due {
                    return;
                }
            }
        }
    }

    let mut vals = Vec::new();
    for i in 0..8 {
        let mut temp = Vec::new();
        for j in 0..4 {
            temp.push((0.0, i, j));
        }
        vals.append(&mut temp);
    }
    let mut temp = Vec::new();
    for j in 0..4 {
        temp.push((0.0, b'u', j));
    }
    vals.append(&mut temp);
    let mut temp = Vec::new();
    for j in 0..4 {
        temp.push((0.0, b'r', j));
    }
    vals.append(&mut temp);
    for i in 0..8 {
        for j in 0..if i == 0 || i == 7 { 2 } else { 3 } {
            if let Some(val) = player_obj.get_rent(j, i, 7) {
                vals[i as usize * 4 + j] =
                    (val, vals[i as usize * 4 + j].1, vals[i as usize * 4 + j].2);
            }
        }
    }
    for j in 0..2 {
        if let Some(val) = player_obj.get_rent(j, b'u', 7) {
            vals[8 as usize * 4 + j] =
                (val, vals[8 as usize * 4 + j].1, vals[8 as usize * 4 + j].2);
        }
    }
    for j in 0..4 {
        if let Some(val) = player_obj.get_rent(j, b'r', 7) {
            vals[9 as usize * 4 + j] =
                (val, vals[9 as usize * 4 + j].1, vals[9 as usize * 4 + j].2);
        }
    }
    vals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut index = 0;
    while player_obj.money < due && index < vals.len() {
        if vals[index].0 == 0.0 {
            index += 1;
            continue;
        }
        player_obj.mortgage(vals[index].2, vals[index].1);
        index += 1;
    }
}

pub fn build_or_not(player: &mut Player) {
    for t in (0..8).rev() {
        if player.check_monopoly(t) {
            let num_prop = get_size_of_tier(t);
            let num_houses = (player.money / (t as i32 / 2 * 50 + 50) as f64) as usize;
            if num_houses < 1 {
                continue;
            }
            for p in 0..num_houses {
                player.make_building(1, t, num_prop - p % num_prop - 1);
            }
        }
    }
}

pub fn unmortgage_or_not(player: &mut Player) {
    for t in (0..8).rev() {
        for p in 0..get_size_of_tier(t) {
            if player.is_mortgaged(t, p) {
                
            }
        }
    }
}
