fn main() {
    let input = std::fs::read_to_string("inputs\\input_2023_04.txt").unwrap();
    println!("{}", wining_ticket(&input))
}

fn wining_ticket(input: &str) -> u32 {
    let mut total_point = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        total_point += wining_card(line);
    }

    total_point
}
fn wining_card(input: &str) -> u32 {
    let mut card_point = 0;
    let numbers = input.split_once(":").unwrap().1;
    let (wining, playing) = numbers.split_once("|").unwrap();
    let winning_number: Vec<&str> = wining.split_whitespace().collect();
    let playing_number: Vec<&str> = playing.split_whitespace().collect();

    for w_nb in &winning_number {
        for p_nb in &playing_number {
            if p_nb == w_nb {
                if card_point == 0 {
                    card_point += 1;
                } else {
                    card_point *= 2;
                }
            }
        }
    }

    card_point
}

fn matches(line: &str) -> u32 {
    let mut card_count = 0;
    let numbers = line.split_once(":").unwrap().1;
    let (wining, playing) = numbers.split_once("|").unwrap();
    let winning_number: Vec<&str> = wining.split_whitespace().collect();
    let playing_number: Vec<&str> = playing.split_whitespace().collect();

    for w_nb in &winning_number {
        for p_nb in &playing_number {
            if p_nb == w_nb {
                card_count += 1;
            }
        }
    }

    card_count
}

fn total_card_count(input:&str)->u32{
    

    0
}