use crate::*;
use crate::keywords;

/*[
    SUMMON(SELF) -> 
    EXIST(SELF) -> GRANT(ALLY, [+1|+1, KEYWORDS[]]),
]*/

pub fn code2card(code: &str) -> Result<Card, Error> {
    Ok(match code {
        "01DE001" => Card {
            cardtype: CardType::Follower,
            name: "Vanguard Bannerman",
            description: "Allegiance: Grant all allies +1|+1",
            keywords: vec![&keywords::CANTBLOCK],
            faction: Faction::Demacia,
            rarity: Rarity::Rare,
            cost: 4,
            attack_point: 3,
            defense_point: 3,
            event_listener: |event: &Event, card_index: CardIndex, game: &Game| {
                match event {
                    Event::Summon(target_card_index) if *target_card_index == card_index => {
                        let player = game.owner(card_index);
                        let allies = player.summons();
                        let ally_grant_events = allies
                            .iter()
                            .map(|ally: &CardIndex| {
                                 Event::Grant(
                                     Effect{
                                         target_card_index: *ally, 
                                         attack_point: 1,
                                         defense_point: 1,
                                         duration: Duration::CardPlaying(card_index), 
                                         ..Default::default()
                                     })})
                        .collect::<Vec<_>>();
                        Some(ally_grant_events)
                    },
                    _ => None,
                }
            },
        },
        _ => return Err(Error::CardCodeNotFound{code: code.to_string()}),
    })
}
