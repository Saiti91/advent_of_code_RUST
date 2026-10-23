fn main() {
    let input = std::fs::read_to_string("inputs\\2024\\input_2024_03.txt").unwrap();
    println!("{}", resoudre(input));
}

fn resoudre(input: String) -> u32 {
    //  reste <- &line[i..]
    //     si reste commence par "do()"         -> enable <- vrai
    //     sinon si reste commence par "don't()" -> enable <- faux
    //     sinon si enable                        -> total += check_rest(reste)
    let mut total = 0;
    let mut enable = true;
    for line in input.lines() {
        for (i, _) in line.char_indices() {
            if line[i..].starts_with("do()") {
                enable = true;
            }
            if line[i..].starts_with("don't()") {
                enable = false
            }
            if enable {
                total += check_rest(&line[i..], &mut enable);
            }
        }
    }
    total
}

fn check_rest(input: &str, enable: &mut bool) -> u32 {
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    //check_rest(reste) :
    //     si reste ne commence pas par "mul(" -> 0
    //     apres <- reste sans les 4 premiers caractères          (&reste[4..])
    //     couper apres au premier ')' -> dedans                  (sinon -> 0)
    //     couper dedans au premier ',' -> gauche, droite         (sinon -> 0)
    //     si gauche et droite sont des nombres de 1 à 3 chiffres -> gauche * droite
    //     sinon -> 0

    if !input.starts_with("mul(") {
        return 0;
    }
    let tronc_input = &input[4..];

    if !tronc_input.contains(')') {
        return 0;
    }
    let (tronc_after_input, _) = tronc_input.split_once(')').unwrap();

    if !tronc_after_input.contains(',') {
        return 0;
    }
    let (gauche, droite) = tronc_after_input.split_once(',').unwrap();

    //x <- 0
    // pour chaque caractère c de gauche :
    //     si c n'est pas un chiffre -> return 0
    //     x <- x * 10 + valeur de c
    if gauche.len() >= 1 && gauche.len() <= 3 {
        for c in gauche.chars() {
            if !c.is_ascii_digit() {
                return 0;
            }
            x = x * 10 + c.to_digit(10).unwrap();
        }
    }
    if droite.len() >= 1 && droite.len() <= 3 {
        for c in droite.chars() {
            if !c.is_ascii_digit() {
                return 0;
            }
            y = y * 10 + c.to_digit(10).unwrap();
        }
    }

    x * y
}
