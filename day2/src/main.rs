use std::{error::Error, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = std::fs::read_to_string("input")?;

    let available_cubes = [Cubes::Red(12), Cubes::Green(13), Cubes::Blue(14)];
    let sum_of_ids: u32 = input
        .lines()
        .map(|line| Game::from_str(line).expect("Failed to parse game"))
        .filter(|game| game.is_possible(&available_cubes))
        .map(|game| game.id)
        .sum();

    println!("{}", sum_of_ids);

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
    fn is_possible(&self, available_cubes: &[Cubes]) -> bool {
        fn cube_amount_cmp(a: &Cubes, b: &Cubes) -> std::cmp::Ordering {
            match (a, b) {
                (Cubes::Red(amount_a), Cubes::Red(amount_b)) => amount_a.cmp(amount_b),
                (Cubes::Green(amount_a), Cubes::Green(amount_b)) => amount_a.cmp(amount_b),
                (Cubes::Blue(amount_a), Cubes::Blue(amount_b)) => amount_a.cmp(amount_b),
                _ => std::cmp::Ordering::Equal,
            }
        }
        fn enough_available(cubes: &Cubes, available_cubes: &[Cubes]) -> bool {
            let possible = !available_cubes.iter().any(|e| {
                cube_amount_cmp(e, cubes) == std::cmp::Ordering::Less
            });
            possible
        }

        let possible = !self.rounds.iter().any(|round| {
            round
                .iter()
                .any(|cubes| !enough_available(cubes, available_cubes))
        });
        possible
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 100: 11 red, 1 blue, 2 green; 3 red, 3 green; 1 blue, 8 red, 4 green; 5 green, 5 blue, 1 red; 2 green, 1 red, 6 blue; 2 green, 8 red, 1 blue
        // split on :
        let (game, rounds) = match s.split(':').collect::<Vec<&str>>()[..] {
            [game, rounds] => (game, rounds),
            _ => {
                return Err(());
            }
        };

        // split on space
        let game_id = match game.split(' ').collect::<Vec<&str>>()[..] {
            ["Game", game_id] => u32::from_str(game_id).expect("Failed to parse game id"),
            _ => {
                return Err(());
            }
        };

        let rounds: Vec<Vec<Cubes>> = rounds
            .split(';')
            .map(|round| {
                round
                    .split(',')
                    .map(|cubes| Cubes::from_str(cubes).expect("Failed to parse cube"))
                    .collect()
            })
            .collect();
        Ok(Game {
            id: game_id,
            rounds,
        })
    }
}

impl FromStr for Cubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split(' ').collect::<Vec<&str>>()[..] {
            [amount, "red"] => Ok(Cubes::Red(
                u32::from_str(amount).unwrap_or_else(|e| panic!("parse amount of green {}", e)),
            )),
            [amount, "green"] => Ok(Cubes::Green(
                u32::from_str(amount).unwrap_or_else(|e| panic!("parse amount of green {}", e)),
            )),
            [amount, "blue"] => Ok(Cubes::Blue(
                u32::from_str(amount).unwrap_or_else(|e| panic!("parse amount of blue {}", e)),
            )),
            _ => panic!("Failed to parse {}", s),
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
