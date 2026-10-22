const BASE: u32 = 10;

fn main() {
    let input = std::fs::read_to_string("inputs\\2023\\input_2023_01.txt").unwrap();
    println!("{}", get_two_digit_code(input));

}

fn get_two_digit_code(input: String) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        let mut found_first_char: bool = false;
        for (i,c) in line.char_indices() {
            //test si le premier chiffre n'a pas été trouvé => test text
            //je cherche une valeur ici (mot, sinon chiffre)  -> je la mets dans VALUE (0 si rien)
            //
            // SI VALUE n'est pas 0 :
            //     si je n'ai pas encore de premier -> PREMIER = VALUE
            //     sinon                            -> DERNIER = VALUE

            let mut value = word_value(&line[i..]);
            if c.is_ascii_digit() {
                value = c.to_digit(BASE).unwrap();
            }
            if value!=0{
                if !found_first_char{
                    first_digit = value;
                    last_digit = value;
                    found_first_char = true;
                }
                else {
                    last_digit = value;
                }
            }
        }
        total += first_digit * 10 + last_digit;

    }
    total
}

fn word_value(reste: &str) -> u32 {
    if reste.starts_with("one") {
        1
    } else if reste.starts_with("two") {
        2
    } else if reste.starts_with("three") {
        3
    } else if reste.starts_with("four") {
        4
    } else if reste.starts_with("five") {
        5
    } else if reste.starts_with("six") {
        6
    } else if reste.starts_with("seven") {
        7
    } else if reste.starts_with("eight") {
        8
    } else if reste.starts_with("nine") {
        9
    } else {
        0
    }
}