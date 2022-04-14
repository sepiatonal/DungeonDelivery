use rand::Rng;

use crate::party::*;
use crate::battler::*;

//struct

pub struct BattleScene {
    pub active_party: PlayerParty,
    pub enemy_units: Vec<EnemyUnit>,
    pub action_queue: Vec<QueuedAction>,
}

pub struct EnemyUnit {
    pub name: String,
    pub battle: Battler,
    pub actions: Vec<(Action, u64 /*action weight*/)>,
    pub cur_action: Option<Action>
}

pub struct QueuedAction {
    initiative: i16,
    raw_result: i16, /*used for things like attack rolls or flat chances of effect*/
    user_index: Option<(usize, Allegience)>,
    targets: Vec<(usize, Allegience)>,
    effect: SkillEffect, /*type: fn(&mut Battler, i16)*/
}

#[derive(Clone)]
pub struct Action {
    weight: u64, /*Skill weight: used to decide which to use*/
    target_allegience: Allegience,
    target_type: TargetType,
    accuracy_value: fn(&Battler) -> i16,
    effect: fn(&mut Battler, i16), /*type: fn(&mut Battler, i16)*/
}

#[derive(Clone)]
pub struct TargetType {
    affect_allies: bool,
    default_allies: bool,
    area_type: AreaType

}

//enums
#[derive(Clone)]
pub enum AreaType {
    Single,
    Row,
    Column
}

//trait

//impl

impl Battler {
    pub fn roll_initiative(&self) -> i16 {
        self.params.speed + crate::utils::exploding_die(6)
    }
    pub fn roll_attack(&self) -> i16 {
        self.params.attack + crate::utils::exploding_die(6)
    }
    pub fn roll_defense(&self) -> i16 {
        self.params.defense + crate::utils::exploding_die(6)
    }
}

impl QueuedAction {

    pub fn call_effect(&self, battler: &mut Battler){
        (self.effect)(battler, self.raw_result);
    }
}

impl EnemyUnit {

    pub fn get_weight(&self, index: usize) -> u64 {
        return self.actions[index].1;
    }

    pub fn get_target(&self) -> usize {
        todo!()
    }

    pub fn decide_action(&mut self) {
        /*I need to take advantage of the rand::distributions::WeightedChoice struct here */
        let mut weightsum: u64 = 0;
        for (i, a) in self.actions.iter().enumerate() {
            weightsum += self.get_weight(i);
        }

        let mut random:u64 = rand::thread_rng().gen_range(0..weightsum);
        if self.actions.is_empty() { self.cur_action = None; }

        for (i, a) in self.actions.iter().enumerate() {
            if random >= self.get_weight(i) { 
                random -= self.get_weight(i);
            } else {
                self.cur_action = Some(a.0.clone());
                break
            }
        }
    }

    pub fn emit_action(&self) -> Option<QueuedAction> {

        if self.cur_action.is_some() {
            let action = self.cur_action.as_ref().unwrap();
            return Some(QueuedAction{
                initiative: self.battle.roll_initiative(),
                raw_result: (action.accuracy_value)(&self.battle),
                user_index: None,
                targets: Vec::new(),
                effect: action.effect
            });
        } else {
            return None;
        }
    }
}

impl HasHP for EnemyUnit {
    fn get_hp(&self) -> i16 {
        todo!()
    }

    fn set_hp(&mut self, val: i16) {
        todo!()
    }

    fn die(&mut self) {
        /*TODO*/
        todo!()
    }
}

impl BattleScene {
    pub fn collect_actions(&mut self) {
        todo!();
    }

    pub fn get_targets_for_enemy(&self, index: usize) -> Vec<(usize, Allegience)>{
        /*way to get targets for a chosen enemy*/
        todo!();
    }

    pub fn get_action_from_enemy(&self, index: usize) -> Option<QueuedAction> { /*and set source/targets*/
        if index < self.enemy_units.len() {
            let maybe_returned_action = self.enemy_units[index].emit_action();
            if maybe_returned_action.is_some() {
                let mut returned_action = maybe_returned_action.unwrap();
                returned_action.user_index = Some((index, Allegience::Enemy));
                returned_action.targets = self.get_targets_for_enemy(index);
                return Some(returned_action);
            }
        }

        return None;
    }

    pub fn sort_initiative(&mut self) {
        self.action_queue.sort_by_key(|x| x.initiative);
    }

    pub fn pop_eval_top_action(&mut self) -> bool { /*this could maybe be updated to return animation / result data?*/
        let maybe_action = self.action_queue.pop();

        if maybe_action.is_some() {
            let action = maybe_action.unwrap();
            let targets: Vec<Battler> = Vec::new();
            
            for t in action.targets.iter() {
                let target: &mut Battler = match t {
                    (i, Allegience::Ally) => &mut self.active_party.members[*i].0.battle,
                    (i, Allegience::Enemy) => &mut self.enemy_units[*i].battle
                };

                action.call_effect(target);
            }

            return true;
        } else {
            return false;
        }
    }

    pub fn evaluate_targets(&mut self, action: QueuedAction) {

    }
}