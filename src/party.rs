//struct

#[derive(Clone, Copy)]
pub struct BattleParameters { /*A way to store parameter modifiers and base values*/
    pub max_hp: i16,
    pub attack: i16,
    pub defense: i16,
    pub strength: i16,
    pub speed: i16,
    pub armor: f64,
}

pub struct PlayerUnit {
    pub cur_hp: i16,
    pub base_battle_parameters: BattleParameters,
    pub battle_parameters: BattleParameters,
    pub status: Status
}

//trait

trait HasParameters {
    fn refresh_parameters(&mut self);
}

trait CanBattle: HasHP {
    fn attack(&self, target: &mut impl CanBattle, acc: i16);
    fn damage(&self, target: &mut impl HasHP, damage: i16);

    fn can_act(&self) -> bool;

    fn get_initiative(&self) -> i16;
    fn roll_attack(&self) -> i16;
    fn roll_defense(&self) -> i16;

    fn evaluate_effect(&self, eff: AbilityEffect, target: &mut impl CanBattle) {
        match eff {
            AbilityEffect::Attack(acc) => self.attack(target, acc),
            AbilityEffect::Damage(dam) => self.damage(target, dam)
        }
    }

    fn on_turn_start(&mut self);

    fn die(&mut self);
}

trait HasHP {
    fn get_hp(&self) -> i16;
    fn set_hp(&mut self, val: i16);
    fn take_damage(&mut self, val: i16) {
        if val > 0 { self.set_hp(self.get_hp() - val); }
    }
    fn take_nonlethal_damage(&mut self, val:i16);
}

trait HasStatus {
    fn add_state(state: Status);
}

//impl

impl BattleParameters {
    fn new() -> BattleParameters {
        BattleParameters {max_hp: 0, attack: 0, defense: 0, strength: 0, speed: 0, armor: 0.00}
    }

    fn add(x: BattleParameters, y: BattleParameters) -> BattleParameters {
        return BattleParameters {
            max_hp: x.max_hp + y.max_hp,
            attack: x.attack + y.attack,
            defense: x.defense + y.defense,
            strength: x.strength + y.strength,
            speed: x.speed + y.speed,
            armor: x.armor + y.armor
        }
    }
}

impl HasHP for PlayerUnit {
    fn get_hp(&self) -> i16 { return self.cur_hp; }

    fn set_hp(&mut self, val: i16) {
        let max_hp = self.battle_parameters.max_hp;
        
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

    fn take_nonlethal_damage(&mut self, val: i16) {
        if val > 0 {
            if self.cur_hp - val >= 1 {
                self.take_damage(val);
            } else {
                self.take_damage(self.cur_hp - 1);
            }
        }
    }
}

impl HasParameters for PlayerUnit{
    fn refresh_parameters(&mut self) {
        self.battle_parameters = self.base_battle_parameters;
        /* TODO: Iterate over equipment and add its stats directly to the player unit */
        todo!();
    }
}

impl CanBattle for PlayerUnit {
    fn attack(&self, target: &mut impl CanBattle, acc: i16) {
        let result = acc + self.roll_attack() - self.roll_defense();
        if result >= 12 {
            target.take_damage(3);
        } else if result >= std::cmp::max(12 - self.battle_parameters.strength, 2) {
            target.take_damage(2);
        } else if result > 0 {
            target.take_damage(1);
        }
    }

    fn can_act(&self) -> bool {
        match self.status {
            Status::Dead => false,
            Status::Paralyzed => false,
            Status::Petrified => false,
            _ => true
        }
    }

    fn get_initiative(&self) -> i16 {
        return self.battle_parameters.speed + crate::utils::exploding_die(6);
    }

    fn roll_attack(&self) -> i16 {
        return self.battle_parameters.attack + crate::utils::exploding_die(6);
    }

    fn roll_defense(&self) -> i16 {
        return self.battle_parameters.defense + crate::utils::exploding_die(6);
    }

    fn damage(&self, target: &mut impl HasHP, damage: i16) {
        target.take_damage(damage);
    }

    fn on_turn_start(&mut self) {
        if self.status == Status::Poisoned {
            self.take_nonlethal_damage(1)
        }
    }

    fn die(&mut self) {
        /*TODO: handle death here. */
        todo!()
    }
}

//enums
pub enum AbilityEffect {
    /*A way to encapsulate a variety of skill/action effects as data*/
    /*Should this part of the data include targeting information? That might be making it do too much. */
    Attack(i16), /*Accuracy*/
    Damage(i16), /*Damage Done*/
}

#[derive(PartialEq)]
pub enum Status {
    Normal,
    Dead,
    Poisoned,
    Petrified,
    Paralyzed
}