use std::fs;
use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

// game description:
// bags of cubes
// you are shown several hands (; seperated) containing multiple cubes of different colors
// you have to calculate the maximum number of cubes for each color to find all the bags/games
// where it was possible to play with a given set of cubes (e.g how many games had less than 13 red
// cubes?)

fn main() -> Result<(), Box<dyn Error>>{
    println!("Cube Conundrum");
    let day_input = fs::read_to_string("./input/day-two.txt")?;

    let total_cubes = HashMap::<Color, usize>::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14)
    ]);

    let sum_of_ids: usize = day_input
        .lines()
        .map(|game_string| read_game_string(game_string.to_string()))
        .filter(|game| game_possible(max_cubes(game.clone()), total_cubes.clone()))
        .map(|game| game.id)
        .sum();

    println!("{}", sum_of_ids);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
struct Cubes {
    color: Color,
    count: usize
}

#[derive(Debug, PartialEq, Clone)]
struct Hand {
    cubes: Vec<Cubes>
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: usize,
    hands: Vec<Hand>
}

#[derive(Debug)]
pub struct ColorError {
    pub message: String
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ColorError {}


impl FromStr for Color {
    type Err = ColorError;

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(ColorError {
                message: format!("Could not derive color from string '{}'", input)
            })
        }
    }
}

fn read_game_id(game_id_string: String) -> usize {
    game_id_string.split(" ").last().unwrap().parse::<usize>().unwrap()
}

fn read_all_hands(hands_string: String) -> Vec::<Hand> {
    hands_string.split("; ")
        .map(|hand_string| {
            let all_cubes = hand_string.split(", ")
                .map(|cubes_string| {
                    let mut cubes_string = cubes_string.split(" ");
                    let count_string = cubes_string.next();
                    let color_string = cubes_string.next();
                    let cs = color_string.unwrap();

                    Cubes {
                        count: count_string.unwrap().parse::<usize>().unwrap(),
                        color: Color::from_str(cs).unwrap()
                    }
                }).collect();

            Hand { cubes: all_cubes }
        }).collect()
}

fn read_game_string(game_string: String) -> Game {
    let mut first_split = game_string.split(": ");
    let game_id = read_game_id(first_split.next().unwrap().to_string());

    let all_hands = read_all_hands(first_split.next().unwrap().to_string());


    Game { id: game_id, hands: all_hands }
}

fn max_cubes(game: Game) -> HashMap<Color, usize> {
    let mut max_cubes: HashMap<Color, usize> = HashMap::new();

    let cubes = game.hands
        .into_iter()
        .map(|hand| hand.cubes)
        .flatten();

    for c in cubes {
        if max_cubes.contains_key(&c.color) {
            if c.count > *max_cubes.get(&c.color).unwrap() {
                max_cubes.insert(c.color, c.count);
            }
        } else {
            max_cubes.insert(c.color, c.count);
        }
    }

    max_cubes
}

/// compare the hands played in a game to see if they were possible to play with a given set of
/// cubes.
fn game_possible(cubes: HashMap<Color, usize>, max_cubes: HashMap<Color, usize>) -> bool {
    *cubes.get(&Color::Red).unwrap() <= *max_cubes.get(&Color::Red).unwrap() &&
    *cubes.get(&Color::Blue).unwrap() <= *max_cubes.get(&Color::Blue).unwrap() &&
    *cubes.get(&Color::Green).unwrap() <= *max_cubes.get(&Color::Green).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_id() {
        assert_eq!(read_game_id("Game 1".to_string()), 1);
        assert_eq!(read_game_id("Game 15".to_string()), 15);
        assert_eq!(read_game_id("Game 23".to_string()), 23);
    }

    // test fixture provided as a function because I can't assign this with const/static
    fn example_game() -> Game {
        Game {
            id: 7,
            hands: vec![
                Hand {
                    cubes: vec![
                        Cubes { color: Color::Blue, count: 3 },
                        Cubes { color: Color::Red, count: 4 },
                    ]
                },

                Hand {
                    cubes: vec![
                        Cubes { color: Color::Red, count: 1 },
                        Cubes { color: Color::Green, count: 2 },
                        Cubes { color: Color::Blue, count: 6 },
                    ]
                },

                Hand {
                    cubes: vec![
                        Cubes { color: Color::Green, count: 2 },
                    ]
                }
            ]
        }
    }

    #[test]
    fn full_game_string_test() {
        assert_eq!(
            read_game_string("Game 7: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            example_game()
        )
    }

    #[test]
    fn max_cubes_test() {
        assert_eq!(
            max_cubes(example_game()),
            HashMap::<Color, usize>::from([
                (Color::Red, 4),
                (Color::Blue, 6),
                (Color::Green, 2),
            ])
        )
    }

    #[test]
    fn game_possible_test() {
        assert!(
            game_possible(
                max_cubes(example_game()),
                HashMap::<Color, usize>::from([
                    (Color::Red, 4),
                    (Color::Blue, 6),
                    (Color::Green, 2),
                ]))
        );

        assert_eq!(
            game_possible(
                max_cubes(example_game()),
                HashMap::<Color, usize>::from([
                    (Color::Red, 4),
                    (Color::Blue, 1),
                    (Color::Green, 2),
                ])),
                false
        );
    }
}
