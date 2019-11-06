use crate::*;
pub const BURST:Keyword = Keyword {
    name: "Burst",
    description: "",
    event_listener: |event: &Event, card_index: CardIndex, game: &Game| {
        None
    }
};
pub const CANTBLOCK:Keyword = Keyword {
    name: "Can't Block",
    description: "",
    event_listener: |event: &Event, card_index: CardIndex, game: &Game| {
        None
    }
};
pub const EPHEMERAL:Keyword = Keyword {
    name: "Ephemeral",
    description: "",
    event_listener: |event: &Event, card_index: CardIndex, game: &Game| {
        None
    }
};
