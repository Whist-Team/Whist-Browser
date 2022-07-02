use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::iter::Map;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuitError {
    InvalidName(String),
    InvalidIndex(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankError {
    InvalidName(String),
    InvalidIndex(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardError {
    Suit(SuitError),
    Rank(RankError),
}

impl From<SuitError> for CardError {
    fn from(error: SuitError) -> Self {
        Self::Suit(error)
    }
}

impl From<RankError> for CardError {
    fn from(error: RankError) -> Self {
        Self::Rank(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct CardComponent(Card);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardVariant {
    Back,
    Front(Card),
}

impl Display for CardVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CardVariant::Back => write!(f, "Back"),
            CardVariant::Front(card) => write!(f, "{}", card),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl CardEnum for Card {
    const COUNT: u8 = Suit::COUNT * Rank::COUNT;
}

impl TryFrom<u8> for Card {
    type Error = CardError;

    fn try_from(idx: u8) -> Result<Self, Self::Error> {
        let suit = Suit::try_from(idx / Rank::COUNT)?;
        let rank = Rank::try_from(idx % Rank::COUNT)?;
        Ok(Self { suit, rank })
    }
}

#[allow(clippy::from_over_into)]
impl Into<u8> for Card {
    fn into(self) -> u8 {
        let suit: u8 = self.suit.into();
        let rank: u8 = self.rank.into();
        suit * Rank::COUNT + rank
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn symbol(&self) -> char {
        match self {
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }
}

impl CardEnum for Suit {
    const COUNT: u8 = 4;
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "clubs"),
            Suit::Diamonds => write!(f, "diamonds"),
            Suit::Hearts => write!(f, "hearts"),
            Suit::Spades => write!(f, "spades"),
        }
    }
}

impl TryFrom<u8> for Suit {
    type Error = SuitError;

    fn try_from(idx: u8) -> Result<Self, Self::Error> {
        match idx {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(SuitError::InvalidIndex(idx)),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u8> for Suit {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<&str> for Suit {
    type Error = SuitError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Suit {
    type Error = SuitError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl FromStr for Suit {
    type Err = SuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clubs" => Ok(Suit::Clubs),
            "diamonds" => Ok(Suit::Diamonds),
            "hearts" => Ok(Suit::Hearts),
            "spades" => Ok(Suit::Spades),
            _ => Err(SuitError::InvalidName(s.into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rank {
    #[serde(rename = "2")]
    Num2,
    #[serde(rename = "3")]
    Num3,
    #[serde(rename = "4")]
    Num4,
    #[serde(rename = "5")]
    Num5,
    #[serde(rename = "6")]
    Num6,
    #[serde(rename = "7")]
    Num7,
    #[serde(rename = "8")]
    Num8,
    #[serde(rename = "9")]
    Num9,
    #[serde(rename = "10")]
    Num10,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn symbol(&self) -> char {
        match self {
            Rank::Num2 => '2',
            Rank::Num3 => '3',
            Rank::Num4 => '4',
            Rank::Num5 => '5',
            Rank::Num6 => '6',
            Rank::Num7 => '7',
            Rank::Num8 => '8',
            Rank::Num9 => '9',
            Rank::Num10 => '0',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }
}

impl CardEnum for Rank {
    const COUNT: u8 = 13;
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Num2 => write!(f, "2"),
            Rank::Num3 => write!(f, "3"),
            Rank::Num4 => write!(f, "4"),
            Rank::Num5 => write!(f, "5"),
            Rank::Num6 => write!(f, "6"),
            Rank::Num7 => write!(f, "7"),
            Rank::Num8 => write!(f, "8"),
            Rank::Num9 => write!(f, "9"),
            Rank::Num10 => write!(f, "10"),
            Rank::Jack => write!(f, "jack"),
            Rank::Queen => write!(f, "queen"),
            Rank::King => write!(f, "king"),
            Rank::Ace => write!(f, "ace"),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = RankError;

    fn try_from(idx: u8) -> Result<Self, Self::Error> {
        match idx {
            0 => Ok(Rank::Num2),
            1 => Ok(Rank::Num3),
            2 => Ok(Rank::Num4),
            3 => Ok(Rank::Num5),
            4 => Ok(Rank::Num6),
            5 => Ok(Rank::Num7),
            6 => Ok(Rank::Num8),
            7 => Ok(Rank::Num9),
            8 => Ok(Rank::Num10),
            9 => Ok(Rank::Jack),
            10 => Ok(Rank::Queen),
            11 => Ok(Rank::King),
            12 => Ok(Rank::Ace),
            _ => Err(RankError::InvalidIndex(idx)),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u8> for Rank {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<&str> for Rank {
    type Error = RankError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Rank {
    type Error = RankError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl FromStr for Rank {
    type Err = RankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Num2),
            "3" => Ok(Rank::Num3),
            "4" => Ok(Rank::Num4),
            "5" => Ok(Rank::Num5),
            "6" => Ok(Rank::Num6),
            "7" => Ok(Rank::Num7),
            "8" => Ok(Rank::Num8),
            "9" => Ok(Rank::Num9),
            "10" => Ok(Rank::Num10),
            "jack" => Ok(Rank::Jack),
            "queen" => Ok(Rank::Queen),
            "king" => Ok(Rank::King),
            "ace" => Ok(Rank::Ace),
            _ => Err(RankError::InvalidName(s.into())),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnorderedCards {
    cards: BTreeSet<Card>,
}

impl UnorderedCards {
    pub fn new() -> Self {
        default()
    }
}

impl CardContainer for UnorderedCards {
    fn add(&mut self, card: Card) {
        self.cards.insert(card);
    }

    fn remove(&mut self, card: &Card) -> bool {
        self.cards.remove(card)
    }

    fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    fn len(&self) -> u8 {
        self.cards.len() as u8
    }
}

impl FromIterator<Card> for UnorderedCards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self {
            cards: BTreeSet::from_iter(iter),
        }
    }
}

impl IntoIterator for UnorderedCards {
    type Item = Card;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a UnorderedCards {
    type Item = &'a Card;
    type IntoIter = std::collections::btree_set::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

impl Extend<Card> for UnorderedCards {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        for card in iter {
            self.add(card);
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedCards {
    cards: Vec<Card>,
}

impl OrderedCards {
    pub fn new() -> Self {
        default()
    }
}

impl CardContainer for OrderedCards {
    fn add(&mut self, card: Card) {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
    }

    fn remove(&mut self, card: &Card) -> bool {
        match self.cards.iter().position(|x| x == card) {
            Some(pos) => {
                self.cards.remove(pos);
                true
            }
            _ => false,
        }
    }

    fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    fn len(&self) -> u8 {
        self.cards.len() as u8
    }
}

impl FromIterator<Card> for OrderedCards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut cards = Self::new();
        cards.extend(iter);
        cards
    }
}

impl IntoIterator for OrderedCards {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a OrderedCards {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

impl Extend<Card> for OrderedCards {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        for card in iter {
            self.add(card);
        }
    }
}

trait CardEnum: TryFrom<u8> {
    const COUNT: u8;

    fn all() -> Map<Range<u8>, fn(u8) -> Self> {
        (0..Self::COUNT).map(|i| match i.try_into() {
            Ok(result) => result,
            _ => panic!(),
        })
    }
}

pub trait CardContainer: FromIterator<Card> + IntoIterator<Item = Card> + Extend<Card> {
    fn all() -> Self {
        Self::from_iter(Card::all())
    }

    fn add(&mut self, card: Card);

    fn remove(&mut self, card: &Card) -> bool;

    fn contains(&self, card: &Card) -> bool;

    fn len(&self) -> u8;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::card::{Card, OrderedCards, Rank, Suit, UnorderedCards};

    #[test]
    fn test_card_into_u8() {
        let card = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let actual: u8 = card.into();
        assert_eq!(38, actual);
    }

    #[test]
    fn test_card_format() {
        let card = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        assert_eq!("ace of hearts", format!("{}", card));
    }

    #[test]
    fn test_card_serialize() {
        let card = Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let expected = json!({
            "suit": "hearts",
            "rank": "ace",
        });
        let actual = serde_json::to_value(&card).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unordered_cards_serialize() {
        let cards = UnorderedCards::from_iter([
            Card {
                suit: Suit::Hearts,
                rank: Rank::Num2,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Num4,
            },
        ]);
        let expected = json!({"cards": [
            {"suit": "hearts", "rank": '2'},
            {"suit": "hearts", "rank": '4'}
        ]});
        let actual = serde_json::to_value(&cards).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_ordered_cards_serialize() {
        let cards = OrderedCards::from_iter([
            Card {
                suit: Suit::Hearts,
                rank: Rank::Num2,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Num4,
            },
        ]);
        let expected = json!({"cards": [
            {"suit": "hearts", "rank": '2'},
            {"suit": "hearts", "rank": '4'}
        ]});
        let actual = serde_json::to_value(&cards).unwrap();
        assert_eq!(expected, actual);
    }
}
