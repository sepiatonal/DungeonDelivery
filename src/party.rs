use crate::battler::{ Battler, BattleBaseParameters };

const PARTY_MAX_X: usize = 4;
const PARTY_MAX_Y: usize = 2;

//struct
#[derive(Clone)]
pub struct PlayerUnit {
    pub name: String,
    pub base_params: BattleBaseParameters,
    pub battle: Battler
}

pub struct PlayerParty {
    pub members: Vec<(PlayerUnit, usize, usize)>, /* unit, x, y */
}

impl PlayerUnit {
    pub fn refresh_params(&mut self) {
        self.battle.params = self.base_params.clone();

        // add any modifiers from equipment here.

        self.battle.cur_hp = self.battle.params.max_hp;
    }
}

impl PlayerParty {
    pub fn new() -> PlayerParty {
        return PlayerParty {
            members: Vec::new(),
        }
    }

    pub fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        for (i, (_, x1, y1)) in self.members.iter().enumerate() {
            if &x == x1 && &y == y1 {
                return Some(i);
            }
        }
        return None;
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        return self.index_of(x, y).is_some()
    }

    pub fn add_member(&mut self, unit: &PlayerUnit, x: usize, y: usize) -> bool { /*put some error correction here lol */
        if self.members.len() >= 5 || self.is_occupied(x, y) || x >= PARTY_MAX_X || y >= PARTY_MAX_Y {
            return false;
        } else {
            self.members.push((unit.clone(), x, y));
            return true;
        }
    }

    pub fn remove_member(&mut self, index: usize) -> bool {
        if self.members.len() <= index {
            return false;
        } else {
            self.members.remove(index);
            return true;
        }
    }
}