fn main() {
    let input = std::fs::read_to_string("inputs\\input_2023_03.txt").unwrap();


}

fn add_input_parts(input: String)-> u32 {
    let mut matrice : Vec<Vec<char>> = Vec :: new();
    for line in input.lines() {
        matrice.push(line.chars().collect());
    }

    0
}