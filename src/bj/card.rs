// Tom Lancaster (c) Summer 2019
//
// Blackjack - card.rs

#[derive(Clone)]
pub struct Card {
    pub suit: String,
    pub value: String
}

impl Card {
    pub fn score(&self) -> u32 {
        let number = match self.value.parse::<u32>() {
            Ok(x) => x,
            Err(_) => self.score_for_face_card()
        };
        number
    }

    pub fn name(&self) -> String {
        return format!("{}_{}", &self.value, &self.suit);
    }

    fn score_for_face_card(&self) -> u32 {
        let score = match self.value.as_str() {
            "J" => 10,
            "Q" => 10,
            "K" => 10,
            "A" => 11,
            _ => 0

        };
        score
    }
}

#[test]
fn test_score_for_numbers() {
    let card = Card { suit: "Hearts".into(), value: "2".into() };
    assert_eq!(card.score(), 2);
}

#[test]
fn test_score_for_face_card() {
    let card = Card { suit: "Clubs".into(), value: "J".into() };
    assert_eq!(card.score(), 10);
}

#[test]
fn test_name() {
    let card = Card { suit: "Hearts".into(), value: "10".into() };
    assert_eq!(card.name(), String::from("10_Hearts"));
}

