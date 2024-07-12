use crate::{rank::Rank, suit::Suit};

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn compare_cards<'a>(cards: &'a [Card], flip_card: &Card) -> &'a Card {
        let manilha_card = flip_card.rank.next();
        println!("Manilha card: {:?}", manilha_card);
        cards.iter().max_by(|a, b| a.rank.cmp(&b.rank)).unwrap()
    }

    pub fn format_card(&self) -> String {
        let rank = Card::rank_to_symbol(&self.rank);
        let suit = Card::suit_to_symbol(&self.suit);

        let card_str = format!(
            "╭──────╮\n\
             │{}     │\n\
             │  {}   │\n\
             │      │\n\
             ╰──────╯\n\
             ",
            rank, suit
        );

        card_str.to_string()
    }

    fn rank_to_symbol(rank: &Rank) -> &str {
        match rank {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }

    fn suit_to_symbol(suit: &Suit) -> &str {
        match suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }
}
