extern crate lordeckcodes;
use lordeckcodes::encoder;

pub mod errors;
pub use errors::*;

type EventListener = fn(&Event, CardIndex, &Game) -> Option<Vec<Event>>;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct CardIndex{
    player_index: usize, 
    card_index: usize,
}

impl CardIndex {
    fn player_index(&self) -> usize {
        self.player_index
    }
    fn card_index(&self) -> usize {
        self.card_index
    }
}

#[derive(Clone)]
pub struct Keyword {
    name: &'static str,
    description: &'static str,
    event_listener: EventListener,
}

#[derive(Clone)]
pub enum Faction {
    Demacia,
    Ionia,
}

#[derive(Clone)]
pub enum CardType {
    Spell,
    Follower,
    Champion,
}

#[derive(Clone)]
pub enum SubType {
    Elite,
    None,
}

#[derive(Clone)]
pub enum Rarity {
    Rare,
    Epic,
    Normal,
}

#[derive(Clone)]
pub enum Duration {
    Turn(i32),
    CardPlaying(CardIndex),
    Battle,
    Permanent,
}


#[derive(Clone)]
pub struct Card {
    cardtype: CardType,
    name: &'static str,
    description: &'static str,
    keywords: Vec<&'static Keyword>,
    faction: Faction,
    attack_point: i32,
    defense_point: i32,
    cost: i32,
    rarity: Rarity,
    event_listener: EventListener,
}

#[derive(Default)]
pub struct Player {
    deck: Vec<Card>,
    stack: Vec<CardIndex>,
    cemetry: Vec<CardIndex>,
    hands: Vec<CardIndex>,
    summons: Vec<CardIndex>,
    mana: i32,
    spell_mana: u32,
}

fn faction_int_to_str(int: u32) -> Result<&'static str, Error> {
    Ok(match int {
        0 => "DE",
        1 => "FR",
        2 => "IO",
        3 => "NX",
        4 => "PZ",
        5 => "SI",
        _ => return Err(Error::InvalidFactionInteger(int)),
    })
}

impl Player {
    fn new(deck_code: &str) -> Result<Player, Error> {
        //let deck = encoder::deck_from_code(String::from(deck_code))?;
        let deck = encoder::deck_from_code(String::from(deck_code)).map_err(|_| Error::LorDeckCodesError)?;
        let deck_parsed: Vec<Card> = Vec::new();
        for card_code_count in deck.cards() {
            let card = card_code_count.card();
            let code = format!("{:0>2}", card.set()) 
                + faction_int_to_str(card.faction())? 
                + &format!("{:0>3}", card.number());
            for _ in 0..card_code_count.count() {
                deck_parsed.push(cards::code2card(code));
            }
        }
        Ok(Player{
            deck: deck_parsed,
            ..Default::default()
        })
    }
    fn deck(&self) -> &Vec<Card> {
        &self.deck
    }
    fn card(&self, card_index: CardIndex) -> &Card {
        &self.deck[card_index.card_index()]
    }
    fn summons(&self) -> &Vec<CardIndex> {
        &self.summons
    }
    fn hands(&self) -> &Vec<CardIndex> {
        &self.hands
    }
    fn stack_top(&self) -> Option<CardIndex> {
        (&self.stack).last().map(|top_ref| *top_ref)
    }
    fn draw(&mut self) -> Result<(), Error> {
        let top = self.stack.pop()
            .ok_or(Error::NoCardsInStack)?;
        self.hands.push(top);
        Ok(())
    }
    fn summon(&mut self, card_index: CardIndex) -> Result<(), Error> {
        if self.hands.contains(&card_index) {
            self.consume_mana(self.card(card_index).cost)?;
            self.hands.retain(|x| *x != card_index);
        } else {
            return Err(Error::CardIndexNotFoundInHands(card_index));
        }
        self.summons.push(card_index);
        Ok(())
    }
    fn kill(&mut self, card_index: CardIndex) -> Result<(), Error> {
        if self.summons.contains(&card_index) {
            self.summons.retain(|x| *x != card_index); 
        } else {
            return Err(Error::CardIndexNotFoundInSummons(card_index));
        }
        self.cemetry.push(card_index);
        Ok(())
    }
    fn charge_mana(&mut self, mana: i32) -> Result<(), Error> {
        self.mana += mana;
        Ok(())
    }
    fn consume_mana(&mut self, mana: i32) -> Result<(), Error> {
        if self.mana < mana {
            Err(Error::NotEnoughMana)
        } else {
            self.mana -= mana;
            Ok(())
        }
    }
}

#[derive(Clone)]
struct Effect {
    target_card_index: CardIndex, 
    attack_point: i32, 
    defense_point: i32, 
    cost: i32, 
    keywords: Option<Vec<&'static Keyword>>, 
    duration: Duration
}

impl Default for Effect {
    fn default() -> Effect {
        Effect{target_card_index: Default::default(), attack_point: 0, defense_point: 0, cost: 0, keywords: None, duration: Duration::Permanent,}
    }
}

enum Event {
    RoundStart,
    Draw(CardIndex),
    Summon(CardIndex),
    Grant(Effect),
    Dead(CardIndex),
    PassTurn,
}

trait User {
    fn pick(&self, cards: &[CardIndex]) -> CardIndex;
    fn pick_or_pass(&self, card: &[CardIndex]) -> Option<CardIndex>;
}

struct Game {
    turn: usize,
    current_player_turn: usize,
    players: [Player; 2],
    users: [Box<dyn User>; 2],
    effects: Vec<Effect>,
    traps: Vec<Card>,
    events: Vec<Event>,
}

impl Game {
    fn owner(&self, card_index: CardIndex) -> &Player {
        &self.players[card_index.player_index]
    }
    fn owner_mut(&mut self, card_index: CardIndex) -> &mut Player {
        &mut self.players[card_index.player_index]
    }
    fn card(&self, card_index: CardIndex) -> &Card {
        &self.owner(card_index).card(card_index)
    }
    fn current_turn_player(&self) -> &Player {
        &self.players[self.current_player_turn]
    }
    fn apply_event(&mut self, e: &mut Event) -> Result<(), Error> {
        match e {
            Event::RoundStart => {
                self.players[0].draw()?;
                self.players[1].draw()?;
            },
            Event::Draw(card_index) => {
                self.owner_mut(*card_index).draw()?;
            },
            Event::Summon(card_index) => {
                self.owner_mut(*card_index).summon(*card_index)?;
            },
            Event::Dead(card_index) => {
            },
            Event::Grant(effect) => {
                self.apply_effect(effect)
            },
        };
        Ok(())
    }
    fn apply_effect(&mut self, effect: Effect) {
        let card = self.card(effect.target_card_index);
        card.attack_point += effect.attack_point;
        card.defense_point += effect.defense_point;
        card. += effect.defense_point;
        self.effects.push(effect)
    }
    fn filter_expired_effects(&mut self) {
        for event in events {
            if event.
        }
    }
    fn propagate_event(&mut self, e: &mut Event) -> Result<(), Error> {
        let new_events: Vec<Event> = self.current_turn_player().hands()
            .iter()
            .filter_map(|card_index| 
                (self.card(*card_index).event_listener) (&e, *card_index, &self) )
            .flatten()
            .collect();
        self.events.extend(new_events);
        Ok(())
    }
    fn consume_event(&mut self, e: Event) -> Result<(), Error> {
        self.apply_event(&mut e);
        self.propagate_event(&mut e);
        Ok(())
    }
    fn turn(&self) {
        // choose card to defense, attack, spell.
        // consume all events
        // pass the turn
    }
}


pub mod keywords;
pub mod cards;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
