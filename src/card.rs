use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::card_deck::{Rank, Suit};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Component)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub hide: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit,
            rank,
            hide: false,
        }
    }

    // 可以添加一些辅助方法，例如翻转牌
    pub fn flip(&mut self) {
        self.hide = !self.hide;
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
    deck.push(Card::new(Suit::Joker, Rank::LittleJoker)); // 小王
    deck.push(Card::new(Suit::Joker, Rank::BigJoker)); // 大王
    deck
}

pub fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    deck
}

pub fn get_sprite_index(card: &Card) -> usize {
    // 计算普通牌在图集中的索引
    if card.hide {
        let suit_offset = match card.suit {
            Suit::Club => 390,
            Suit::Diamond => 13,
            Suit::Heart => 0,
            Suit::Spade => 26,
            Suit::Joker => 52,
        };
        let rank_offset = match card.rank {
            Rank::Ace => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
            Rank::LittleJoker => 1,
            Rank::BigJoker => 0,
        };
        suit_offset + rank_offset
    } else {
        54
    }
}
