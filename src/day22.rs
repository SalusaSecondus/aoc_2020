use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{bail, Context, Result};

fn load_decks(file_name: &str) -> Result<(VecDeque<u32>, VecDeque<u32>)> {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();

    let mut first_player = true;
    for line in crate::read_file(file_name)? {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "Player 1:" {
            first_player = true;
            continue;
        }
        if line == "Player 2:" {
            first_player = false;
            continue;
        }
        if first_player {
            player1.push_back(line.parse()?);
        } else {
            player2.push_back(line.parse()?);
        }
    }

    Ok((player1, player2))
}

fn play_turn(player1: &mut VecDeque<u32>, player2: &mut VecDeque<u32>) -> Result<i8> {
    let player1_card = player1.pop_front().context("Player 1 has no cards")?;
    let player2_card = player2.pop_front().context("Player 2 has no cards")?;

    if player1_card > player2_card {
        player1.push_back(player1_card);
        player1.push_back(player2_card);
        if player2.is_empty() {
            Ok(-1)
        } else {
            Ok(1)
        }
    } else {
        player2.push_back(player2_card);
        player2.push_back(player1_card);
        if player1.is_empty() {
            Ok(-2)
        } else {
            Ok(2)
        }
    }
}

fn score_deck(deck: &[u32]) -> u64 {
    let mut result = 0;
    let card_count = deck.len() as u64;

    for (idx, card) in deck.iter().enumerate() {
        result += *card as u64 * (card_count - idx as u64);
    }

    result
}

fn play_recursive_combat(
    player1: &mut VecDeque<u32>,
    player2: &mut VecDeque<u32>,
    memo: &mut HashMap<(VecDeque<u32>, VecDeque<u32>), i8>,
    depth: u32,
) -> Result<i8> {
    // println!("=== Game {} ===\n", depth);
    let mut history: HashSet<(VecDeque<u32>, VecDeque<u32>)> = HashSet::new();

    // println!(">>>Entering game with memo size {} and depth {}", memo.len(), depth);
    let mut _round = 1;
    loop {
        // println!("-- Round {} (Game {}) --", _round, depth);
        // println!("Player 1's deck: {:?}", player1);
        // println!("Player 2's deck: {:?}", player2);
        // println!("Turn {}", _turn);
        let history1 = player1.clone();
        let history2 = player2.clone();
        if !history.insert((history1, history2)) {
            // println!("History grants win with depth {}", depth);
            return Ok(1);
        }

        let card1 = player1.pop_front().context("Player 1 has no cards")?;
        let card2 = player2.pop_front().context("Player 2 has no cards")?;

        // println!("Player 1 plays: {}", card1);
        // println!("Player 2 plays: {}", card2);

        let winner;
        if card1 <= player1.len() as u32 && card2 <= player2.len() as u32 {
            let (memo1, memo2) = (player1.clone(), player2.clone());
            if let Some(memo_winner) = memo.get(&(memo1, memo2)) {
                // println!("Memoized win with size {} and depth {}", memo.len(), depth);
                winner = *memo_winner;
            } else {
                // println!("Playing a sub-game to determine the winner...\n");
                let mut sub1 = VecDeque::new();
                let mut sub2 = VecDeque::new();
                sub1.extend(player1.iter().take(card1 as usize));
                sub2.extend(player2.iter().take(card2 as usize));

                winner =
                    play_recursive_combat(&mut sub1.clone(), &mut sub2.clone(), memo, depth + 1)?;
                // println!("...anyway, back to game {}.", depth);
                memo.insert((sub1, sub2), winner);
            }
        } else {
            winner = if card1 > card2 { 1 } else { 2 };
        }

        // println!("Player {} wins round {} of game {}!\n", winner, _round, depth);
        if winner == 1 {
            player1.push_back(card1);
            player1.push_back(card2);
            if player2.is_empty() {
                // println!("<<<Leaving game with memo size {}", memo.len());
                return Ok(1);
            }
        } else if winner == 2 {
            player2.push_back(card2);
            player2.push_back(card1);
            if player1.is_empty() {
                // println!("<<<Leaving game with memo size {}", memo.len());

                return Ok(2);
            }
        } else {
            bail!("Invalid state");
        }
        _round += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn day22_smoke1() -> Result<()> {
        let (mut player1, mut player2) = load_decks("day22_smoke.txt")?;

        let mut turn_count = 1;
        while play_turn(&mut player1, &mut player2)? > 0 {
            turn_count += 1;
            // NOP
        }
        player1.make_contiguous();
        player2.make_contiguous();
        let winning_score;
        if player1.is_empty() {
            winning_score = score_deck(player2.as_slices().0);
        } else {
            winning_score = score_deck(player1.as_slices().0);
        }

        assert_eq!(306, winning_score);
        assert_eq!(29, turn_count);
        Ok(())
    }

    #[test]
    fn day22_1() -> Result<()> {
        let (mut player1, mut player2) = load_decks("day22.txt")?;

        let mut turn_count = 1;
        while play_turn(&mut player1, &mut player2)? > 0 {
            turn_count += 1;
            // NOP
        }
        player1.make_contiguous();
        player2.make_contiguous();
        let winning_score;
        if player1.is_empty() {
            winning_score = score_deck(player2.as_slices().0);
        } else {
            winning_score = score_deck(player1.as_slices().0);
        }

        assert_eq!(34255, winning_score);
        assert_eq!(133, turn_count);
        Ok(())
    }

    #[test]
    fn day22_smoke2() -> Result<()> {
        let (mut player1, mut player2) = load_decks("day22_smoke.txt")?;
        let mut memo = HashMap::new();
        let result = play_recursive_combat(&mut player1, &mut player2, &mut memo, 1)?;

        player1.make_contiguous();
        player2.make_contiguous();
        let winning_score = match result {
            1 => score_deck(player1.as_slices().0),
            2 => score_deck(player2.as_slices().0),
            _ => bail!("Invalid result"),
        };

        assert_eq!(291, winning_score);

        let (mut player1, mut player2) = load_decks("day22_smoke2.txt")?;

        let result = play_recursive_combat(&mut player1, &mut player2, &mut memo, 1)?;

        player1.make_contiguous();
        player2.make_contiguous();
        let winning_score = match result {
            1 => score_deck(player1.as_slices().0),
            2 => score_deck(player2.as_slices().0),
            _ => bail!("Invalid result"),
        };

        assert_eq!(105, winning_score);

        Ok(())
    }

    #[test]
    fn day22_2() -> Result<()> {
        let (mut player1, mut player2) = load_decks("day22.txt")?;
        let mut memo = HashMap::new();

        let result = play_recursive_combat(&mut player1, &mut player2, &mut memo, 1)?;

        player1.make_contiguous();
        player2.make_contiguous();
        let winning_score = match result {
            1 => score_deck(player1.as_slices().0),
            2 => score_deck(player2.as_slices().0),
            _ => bail!("Invalid result"),
        };

        assert_eq!(33369, winning_score);

        Ok(())
    }
}
