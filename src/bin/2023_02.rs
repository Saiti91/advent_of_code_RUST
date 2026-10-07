fn main() {
    let input = std::fs::read_to_string("inputs\\input_2023_02.txt").unwrap();

    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    //println!("{}",possible_game_add(input, red_count, green_count, blue_count));

    println!("{}",max_game_add(input));
}

fn possible_game_add(input: String, red_count: u32, green_count: u32, blue_count: u32) -> u32 {
    let mut game_possible: bool = true;

    let mut add_total_game: u32 = 0;

    for line in input.lines() {
        let (index, games) = line.split_once(":").unwrap();
        let game_index = index
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        for game in games.split(";") {
            for colours in game.split(",") {
                let colour_: Vec<&str> = colours.split_whitespace().collect();

                let colour = colour_[1];
                let nb_cube = colour_[0].parse::<u32>().unwrap();

                if colour == "red" && nb_cube > red_count {
                    game_possible = false
                }
                if colour == "green" && nb_cube > green_count {
                    game_possible = false
                }
                if colour == "blue" && nb_cube > blue_count {
                    game_possible = false
                }
            }
        }
        if game_possible {
            add_total_game += game_index;
        }
        game_possible = true;
    }
    add_total_game
}

fn max_game_add(input: String) -> u32 {

    let mut add_total_game: u32 = 0;

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for line in input.lines() {
        let (_, games) = line.split_once(":").unwrap();

        for game in games.split(";") {
            for colours in game.split(",") {
                let colour_: Vec<&str> = colours.split_whitespace().collect();

                let colour = colour_[1];
                let nb_cube = colour_[0].parse::<u32>().unwrap();

                if colour == "red"  {
                    if nb_cube > max_red {
                        max_red = nb_cube;
                    }
                }
                if colour == "green" {
                    if nb_cube > max_green {
                        max_green = nb_cube;
                    }
                }
                if colour == "blue" {
                    if nb_cube > max_blue {
                        max_blue = nb_cube;
                    }
                }
            }
        }
        add_total_game += max_red * max_green * max_blue;
        max_red = 0;
        max_blue = 0;
        max_green = 0;
    }
    add_total_game
}