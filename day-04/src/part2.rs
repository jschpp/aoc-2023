use super::my_parser::*;
use std::collections::VecDeque;

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_input, cards) = my_parser(input).unwrap();
    let max_cards = cards.len() as u32;
    let mut cards_touched: u32 = 0;
    let mut todo: VecDeque<Card> = cards.clone().into();
    let winning_count: Vec<u32> = cards
        .iter()
        .map(|c| {
            c.numbers
                .iter()
                .filter(|number| c.winning.contains(number))
                .count() as u32
        })
        .collect();

    //TODO: Still slow but getting better
    while !todo.is_empty() {
        let card = todo
            .pop_front()
            .expect("since todo is not yet empty this must succeed");
        cards_touched += 1;
        let num_correct = winning_count[card.number as usize - 1];
        if num_correct > 0 {
            let r = max_cards.min(card.number + 1)..=max_cards.min(card.number + num_correct);
            // dbg!(&r, card.number);
            cards
                .iter()
                .filter(|x| r.contains(&x.number))
                .for_each(|x| todo.push_back(x.clone()));
        }
    }
    Ok(cards_touched.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
