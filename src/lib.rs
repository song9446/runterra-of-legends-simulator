extern crate lordeckcodes;
/*
enum Action {
    AttackRoundStart{player: Player};
    AttackRoundEnd{player: Player};
    DefendRoundStart{player: Player};
    DefendRoundEnd{player: Player};
    RoundStart;
    RoundEnd;
    AttackStart{attack_card: Card, defend_card: Card};
    AttackEnd{player: Player};
    Stun{target: Card};
}
*/

trait Player {
    fn pick(cards: &[Card]) -> Card;
}

struct Keyword<F: Fn(Player, Player) -> Action> {
    func: F,
}

enum Card<'a, F: Fn() -> Action> {
    Spell{
        spell: Fn(&self, &self, &opponent, &spell) -> i32;
        name: &'a str,
        description: &'a str,
    }
    Unit{
        name: &'a str,
        description: &'a str,
        keywords: &'a [Keyword]
        attack_point: i32,
        defense_point: i32,
    }
}

struct PlayerState {
    deck: Vec<Card>,
    cemetry: Vec<Card>,
    hand: Vec<Card>,
    field: Vec<Card>,
    spell_chain: Vec<Card>, 
}


trait Cost {
    fn cost(&self) -> i32;
}

trait Unit {
    fn attack_point(&self) -> i32;
    fn deffend_point(&self) -> i32;
    fn attack(&self, &opponent)
}

trait Field {
    fn left(&self, target: Card)
}

trait Spell {
    fn spell(&self, &self, &opponent, &spell) -> i32;
}



struct UnitCard {
    cost: i32,
    attack_point: i32,
    defense_point: i32,
}
impl Cost for UnitCard {
    fn cost(&self) -> i32 {
        self.cost
    }
}


struct SpellCard {
    cost: i32,
}
impl 


struct World {
    player1: Player,
    player2: Player,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
