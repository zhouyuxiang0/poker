use bevy::prelude::*;

use crate::common::CardIndex;

#[derive(Component, Debug)]
pub struct Card {
    pub card_index: Option<CardIndex>,
    pub hidden: bool,
}

impl Card {
    pub fn new(card_index: Option<CardIndex>, hidden: bool) -> Self {
        Self { card_index, hidden }
    }
}
