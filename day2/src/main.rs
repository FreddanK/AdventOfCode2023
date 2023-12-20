use std::cmp::max;
use std::str::FromStr;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let input: String = std::fs::read_to_string("input")?;

    let available_cubes = [Cubes::Red(12), Cubes::Green(13), Cubes::Blue(14)];
    let games: Result<Vec<Game>> = input
        .lines()
        .map(|line| Game::from_str(line).with_context(|| format!("Failed to parse game: {}", line)))
        .collect();

    if let Ok(games) = games {
        let sum_of_ids: u32 = games
            .iter()
            .filter(|game| game.is_possible(&available_cubes))
            .map(|game| game.id)
            .sum();

        println!("{}", sum_of_ids);

        let sum_of_powers: u32 = games.iter().map(|game| game.power()).sum();

        println!("{}", sum_of_powers);
    }

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cubes>>,
}

#[derive(Debug)]
enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Game {
    fn power(&self) -> u32 {
        let red = self.max_of(&Cubes::Red(0));
        let blue = self.max_of(&Cubes::Blue(0));
        let green = self.max_of(&Cubes::Green(0));
        red * blue * green
    }

    fn max_of(&self, cubes: &Cubes) -> u32 {
        let mut ret: u32 = 0;
        for round in self.rounds.iter() {
            for cube in round {
                let max = match (cube, cubes) {
                    (Cubes::Red(a), Cubes::Red(b)) => max(a, b),
                    (Cubes::Blue(a), Cubes::Blue(b)) => max(a, b),
                    (Cubes::Green(a), Cubes::Green(b)) => max(a, b),
                    (_, _) => &0,
                };
                if max > &ret {
                    ret = *max;
                }
            }
        }
        ret
    }

    fn is_possible(&self, available_cubes: &[Cubes]) -> bool {
        fn possible(game: &Game, cubes: &Cubes) -> bool {
            match cubes {
                Cubes::Red(available) => game.max_of(&Cubes::Red(0)) <= *available,
                Cubes::Blue(available) => game.max_of(&Cubes::Blue(0)) <= *available,
                Cubes::Green(available) => game.max_of(&Cubes::Green(0)) <= *available,
            }
        }
        for cube in available_cubes {
            if !possible(self, cube) {
                return false;
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 100: 11 red, 1 blue, 2 green; 3 red, 3 green; 1 blue, 8 red, 4 green; 5 green, 5 blue, 1 red; 2 green, 1 red, 6 blue; 2 green, 8 red, 1 blue
        // split on :
        let (game, rounds) = match s.split(':').collect::<Vec<&str>>()[..] {
            [game, rounds] => (game, rounds),
            _ => bail!("Failed to split on :"),
        };

        // split on space
        let game_id = match game.split(' ').collect::<Vec<&str>>()[..] {
            ["Game", game_id] => u32::from_str(game_id).context("Failed to parse game id")?,
            _ => bail!("Failed to split on space"),
        };

        let rounds: Result<Vec<Vec<Cubes>>> = rounds
            .split(';')
            .map(|round| {
                round
                    .split(',')
                    .map(|cubes| Cubes::from_str(cubes).context("Failed to parse cube"))
                    .collect()
            })
            .collect();
        let rounds = rounds?;
        Ok(Game {
            id: game_id,
            rounds,
        })
    }
}

impl FromStr for Cubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split(' ').collect::<Vec<&str>>()[..] {
            [amount, "red"] => Ok(Cubes::Red(
                u32::from_str(amount).context("parse amount of red {}")?,
            )),
            [amount, "green"] => Ok(Cubes::Green(
                u32::from_str(amount).context("parse amount of green {}")?,
            )),
            [amount, "blue"] => Ok(Cubes::Blue(
                u32::from_str(amount).context("parse amount of blue {}")?,
            )),
            _ => bail!("Failed to parse {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cubes, FromStr, Game};

    fn input() -> String {
        String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
    }

    #[test]
    fn test_parse() {
        let games: Vec<Game> = input()
            .lines()
            .map(|line| Game::from_str(line).expect("Failed to parse game"))
            .collect();
        assert_eq!(games.len(), 5);

        println!("{:?}", games);
    }

    #[test]
    fn test_is_possible() {
        let game = Game {
            id: 1,
            rounds: vec![
                vec![Cubes::Red(6)],
                vec![Cubes::Green(5)],
                vec![Cubes::Blue(2)],
            ],
        };
        let many_available_cubes = [Cubes::Red(12), Cubes::Green(13), Cubes::Blue(14)];
        let few_available_cubes = [Cubes::Red(1), Cubes::Green(3), Cubes::Blue(1)];

        assert!(game.is_possible(&many_available_cubes));
        assert!(!game.is_possible(&few_available_cubes));
    }

    #[test]
    fn test_is_possible2() {
        let games: Vec<Game> = input()
            .lines()
            .map(|line| Game::from_str(line).expect("Failed to parse game"))
            .collect();
        let available_cubes = [Cubes::Red(12), Cubes::Green(13), Cubes::Blue(14)];

        assert!(games.get(0).unwrap().is_possible(&available_cubes));
        assert!(games.get(1).unwrap().is_possible(&available_cubes));
        assert!(!games.get(2).unwrap().is_possible(&available_cubes));
        assert!(!games.get(3).unwrap().is_possible(&available_cubes));
        assert!(games.get(4).unwrap().is_possible(&available_cubes));
    }

    #[test]
    fn test_sum() {
        let available_cubes = [Cubes::Red(12), Cubes::Green(13), Cubes::Blue(14)];
        let sum_of_ids: u32 = input()
            .lines()
            .map(|line| Game::from_str(line).expect("Failed to parse game"))
            .filter(|game| game.is_possible(&available_cubes))
            .map(|game| game.id)
            .sum();
        assert_eq!(sum_of_ids, 8);
    }
}
