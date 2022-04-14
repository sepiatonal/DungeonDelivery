use crate::party::BattleParameters;

struct Item {
    parameters: BattleParameters,
    weight: i16,
    volume: i16,
    item_type: ItemType
}

struct Inventory {
    items: Vec<Item>
}

//enums

enum ItemType {
    Weapon,
    Offhand,
    Armor,
    MiscEquip,
    Usable,
    Other
}