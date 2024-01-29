use bevy::prelude::*;

use crate::common::CardIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    /**梅花 */
    Club,
    /**方块 */
    Diamond,
    /**红心 */
    Heart,
    /**黑桃 */
    Spade,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    LittleJoker,
    BigJoker,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    // 可以添加其他与牌相关的功能，如比较大小等
}

// 可以定义一些牌的比较功能，例如：
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank
            .cmp(&other.rank)
            .then_with(|| self.suit.cmp(&other.suit))
    }
}

// 创建一副牌的函数
pub fn new_deck() -> Vec<Card> {
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    let mut deck = Vec::with_capacity(suits.len() * ranks.len() + 2); // 54张牌
    for &suit in suits.iter() {
        for &rank in ranks.iter() {
            deck.push(Card::new(suit, rank));
        }
    }
    // 添加大小王
    deck.push(Card::new(Suit::Spade, Rank::LittleJoker)); // 小王
    deck.push(Card::new(Suit::Spade, Rank::BigJoker)); // 大王
    deck
}

pub fn get_sprite_index(card: &Card) -> usize {
    match card.rank {
        Rank::LittleJoker => 52, // 假设小王在图集中的索引是52
        Rank::BigJoker => 53,    // 假设大王在图集中的索引是53
        _ => {
            // 计算普通牌在图集中的索引
            let suit_offset = match card.suit {
                Suit::Club => 0,
                Suit::Diamond => 13,
                Suit::Heart => 26,
                Suit::Spade => 39,
            };
            let rank_offset = match card.rank {
                Rank::Two => 0,
                Rank::Three => 1,
                Rank::Ace => 12,
                Rank::Four => todo!(),
                Rank::Five => todo!(),
                Rank::Six => todo!(),
                Rank::Seven => todo!(),
                Rank::Eight => todo!(),
                Rank::Nine => todo!(),
                Rank::Ten => todo!(),
                Rank::Jack => todo!(),
                Rank::Queen => todo!(),
                Rank::King => todo!(),
                Rank::LittleJoker => todo!(),
                Rank::BigJoker => todo!(),
            };
            suit_offset + rank_offset
        }
    }
}
