fn main() {
    let input = std::fs::read_to_string("inputs\\input_2023_03.txt").unwrap();
    println!("{}", add_input_parts(input))
}

fn add_input_parts(input: String) -> u32 {
    let mut matrice: Vec<Vec<char>> = Vec::new();
    let mut total = 0;
    for line in input.lines() {
        matrice.push(line.chars().collect());
    }
    let matrice_height = matrice.len();
    let matrice_width = matrice[0].len();
    let mut character_found: bool = false;

    for i in 0..matrice_height {
        let mut total_case = 0;

        for j in 0..matrice_width {
            if matrice[i][j].is_ascii_digit() {
                total_case = total_case * 10 + matrice[i][j].to_digit(10).unwrap();
                if touche_symbole(&matrice, i, j) {
                    character_found = true;
                }
            } else {
                if character_found {
                    total += total_case;
                }
                total_case = 0;
                character_found = false;
            }
        }
        if character_found {
            total += total_case;
        }
    }
    total
}

fn touche_symbole(matrice: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let matrice_height = matrice.len() - 1;
    let matrice_width = matrice[0].len() - 1;

    let ligne_debut = if i == 0 { i } else { i - 1 };
    let ligne_fin = if i == matrice_height { i } else { i + 1 };
    let colonne_debut = if j == 0 { j } else { j - 1 };
    let colonne_fin = if j == matrice_width { j } else { j + 1 };

    for x in ligne_debut..=ligne_fin {
        for y in colonne_debut..=colonne_fin {
            let case = matrice[x][y];
            if !case.is_ascii_digit() && case != '.' {
                return true;
            }
        }
    }
    return false;
}
