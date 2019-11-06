//use lordeckcodes::encoder::LorError;
use crate::*;
pub enum Error {
    CardCodeNotFound{code: String},
    InvalidFactionInteger(u32),
    LorDeckCodesError,//(LorError),
    CardIndexNotFoundInHands(CardIndex),
    CardIndexNotFoundInSummons(CardIndex),
    NoCardsInStack,
    ManaOverFlow,
    NotEnoughMana,
}
