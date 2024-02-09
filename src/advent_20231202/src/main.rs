use regex::Regex;

pub struct Game {
    max_cubes_needed : Set,
}

#[derive(Clone)]
pub struct Set {
    blue : u32,
    green : u32,
    red : u32,
}

fn main() {
    let input = include_str!("input.txt");

    let games: u32 = input.lines().map(|line| {
        create_game_from_line(line)
    }).fold(0, |acc, game| acc + (game.max_cubes_needed.green * game.max_cubes_needed.red * game.max_cubes_needed.blue) );

    println!("Result is {}", games);
}

fn create_game_from_line(line: &str) -> Game {
    let regex_color = Regex::new(r"(\d+) (blue|green|red)").unwrap();
    let mut sets: Vec<Set> = vec![];

    let line_without_id = line.split(":").collect::<Vec<&str>>()[1];

    for set in line_without_id.split(";").collect::<Vec<&str>>() {
        let mut blue_count = 0;
        let mut green_count = 0;
        let mut red_count = 0;
        for captures in regex_color.captures_iter(set) {
            // Extract count and color from captures
            if let Some(count_str) = captures.get(1) {
                if let Some(color_str) = captures.get(2) {
                    // Parse count string to u32
                    if let Ok(count) = count_str.as_str().parse::<u32>() {
                        // Increment count based on color
                        match color_str.as_str() {
                            "blue" => blue_count += count,
                            "green" => green_count += count,
                            "red" => red_count += count,
                            _ => {}
                        }
                    }
                }
            }
        }

        sets.push(Set {
            blue: blue_count,
            green: green_count,
            red: red_count,
        })
    }

    Game {
            max_cubes_needed : extract_max_from_sets(sets.clone())
        }
}

fn extract_max_from_sets(sets : Vec<Set>) -> Set {
    let mut result = Set {
        blue: 0,
        green: 0,
        red: 0,
    };

    sets.iter().for_each(|s| {
        if s.red > result.red {
            result.red = s.red;
        }
        if s.green > result.green{
            result.green = s.green;
        }
        if s.blue > result.blue{
            result.blue = s.blue;
        }
    });

    return result;
}