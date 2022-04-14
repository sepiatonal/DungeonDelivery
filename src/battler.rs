use std::collections::HashSet;

#[derive(Clone)]
pub struct BattleBaseParameters {
    pub max_hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub strength: i16,
    pub speed: i16,
    pub armor: f64,
}

#[derive(Clone)]
pub struct Battler {
    pub params: BattleBaseParameters,
    pub cur_hp: i16,
    pub status_effects: HashSet<StatusEffect>,
}

#[derive(Clone)]
pub struct BattleSkill {
    pub effect: SkillEffect,
    pub targets: Vec<(usize, Allegience)> /*Battler index, allegience*/
}

impl Battler {
    pub fn new() -> Battler {
        Battler {
            params: BattleBaseParameters::new(),
            cur_hp: 0,
            status_effects: HashSet::new()
        }
    }
}

impl BattleBaseParameters {
    pub fn new() -> BattleBaseParameters {
        BattleBaseParameters {max_hp: 0, attack: 0, defense: 0, strength: 0, speed: 0, armor: 0.00}
    }

    pub fn add(&mut self, y: BattleBaseParameters) {
        self.max_hp += y.max_hp;
        self.attack += y.attack;
        self.defense += y.defense;
        self.strength += y.strength;
        self.speed += y.speed;
        self.armor += y.armor;
        
    }
}

pub trait HasHP {
    fn get_hp(&self) -> i16;
    fn set_hp(&mut self, val: i16);
    fn take_damage(&mut self, val: i16) {
        if val > 0 { self.set_hp(self.get_hp() - val); }
    }
    fn take_nonlethal_damage(&mut self, val: i16) {
        if val > 0 {
            if self.get_hp() - val >= 1 {
                self.take_damage(val);
            } else {
                self.take_damage(self.get_hp() - 1);
            }
        }
    }

    fn die(&mut self);
}

impl HasHP for Battler {
    fn get_hp(&self) -> i16 { return self.cur_hp; }

    fn set_hp(&mut self, val: i16) {
        let max_hp = self.params.max_hp;
        
        if val > max_hp {
            self.cur_hp = max_hp;
        }
        else if val <= 0 {
            self.cur_hp = 0;
            self.die();
        }
        else {
            self.cur_hp = val;
        }
    }

    fn take_damage(&mut self, val: i16) {
        if val > 0 { self.set_hp(-val); }
    }

    fn die(&mut self) {
        todo!();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusEffect {
    Normal,
    Dead,
    Poisoned,
    Petrified,
    Paralyzed
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Allegience {
    Ally,
    Enemy
}

pub type SkillEffect = fn(&mut Battler, i16);