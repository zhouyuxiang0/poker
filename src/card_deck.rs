use serde::{Deserialize, Serialize};

use crate::card::Card;

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
    /**王 */
    Joker,
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

pub struct CardDeck {
    cards: Vec<Card>,
}

impl CardDeck {
    pub fn new() -> Self {
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
        let mut cards = Vec::with_capacity(suits.len() * ranks.len() + 2); // 54张牌
        for &suit in suits.iter() {
            for &rank in ranks.iter() {
                cards.push(Card::new(suit, rank));
            }
        }
        // 添加大小王
        cards.push(Card::new(Suit::Joker, Rank::LittleJoker)); // 小王
        cards.push(Card::new(Suit::Joker, Rank::BigJoker)); // 大王
        Self { cards }
    }
}
