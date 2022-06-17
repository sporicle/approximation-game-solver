use rand::thread_rng;
use rand::prelude::SliceRandom;

fn main() {
    let mut deck = prepare_deck();
    deck.shuffle(&mut thread_rng());
    let cards = &deck[0..4].to_vec();
    println!("Cards: {:?}", cards);

    let mut res = reduce_all(cards);
    res.sort();
    res.dedup();

    println!("{:?}",res);
}

fn reduce_all(cards: &Vec<i32>) -> Vec<Vec<i32>>{
    let mut res = reduce(cards);
    for _i in 0..cards.len()-2 {
        let mut new_res = Vec::new();

        for a in res{
            new_res.extend(reduce(&a));
        }
        res = new_res;
    }
    res
}

// Assumes at least 2 cards
fn reduce(cards: &Vec<i32>) -> Vec<Vec<i32>>{
    let mut res = Vec::new();

    for card1 in cards {
        let mut remaining_cards = cards.clone();
        remaining_cards.remove(cards.iter().position(|&r| r == *card1).unwrap());
        for card2 in remaining_cards {
            let new_cards = perform_actions(*card1, card2);
            for new_card in new_cards {
                let mut others = cards.clone();
                others.remove(others.iter().position(|&r| r == *card1).unwrap());
                others.remove(others.iter().position(|&r| r == card2).unwrap());
                others.extend([new_card].iter());
                res.push(others);
            }
        }
    }
    
    res
}

fn prepare_deck() -> Vec<i32> {
    let mut deck = Vec::new();
    for i in 1..14 {
        deck.push(i);
        deck.push(i);
        deck.push(i);
        deck.push(i);
    }

    deck
}

fn perform_actions(card1: i32, card2: i32) -> Vec<i32> {
    let mut res = Vec::new();
    res.push(card1+card2);
    res.push(card1-card2);
    res.push(card1*card2);
    if card2!= 0 && card1%card2 == 0 {
        res.push(card1/card2);
    }

    res
}