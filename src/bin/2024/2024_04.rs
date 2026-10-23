fn main() {
    let input = std::fs::read_to_string("inputs\\2024\\input_2024_04.txt").unwrap();
    println!("{}", resoudre(input));
}

//resoudre(input) :
//     grille <- liste vide
//     pour chaque ligne de input :
//         ajouter à grille la ligne transformée en liste de caractères
//
//     total <- 0
//     pour i de 0 à nombre_de_lignes - 1 :
//         pour j de 0 à nombre_de_colonnes - 1 :
//             total <- total + compter_xmas(grille, i, j)
//     renvoyer total
//
fn resoudre(input: String) -> u32 {
    let mut grille: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grille.push(line.chars().collect());
    }
    let mut total = 0;

    for i in 0..grille.len() - 1 {
        for j in 0..grille.len() - 1 {
            total += compter_xmas(&grille, i, j)
        }
    }
    total
}


// compter_xmas(grille, i, j) :
//     si grille[i][j] n'est pas 'X' -> renvoyer 0
//
//     mot <- ['X', 'M', 'A', 'S']
//     directions <- [(-1,-1), (-1,0), (-1,1),
//                    ( 0,-1),         ( 0,1),
//                    ( 1,-1), ( 1,0), ( 1,1)]
//     trouves <- 0
//
//     pour chaque (dl, dc) dans directions :
//         // position de la 4e lettre
//         l_fin <- i + 3*dl
//         c_fin <- j + 3*dc
//         si l_fin < 0 ou l_fin >= nombre_de_lignes
//            ou c_fin < 0 ou c_fin >= nombre_de_colonnes :
//             passer à la direction suivante
//
//         ok <- vrai
//         pour k de 0 à 3 :
//             si grille[i + k*dl][j + k*dc] != mot[k] :
//                 ok <- faux
//         si ok :
//             trouves <- trouves + 1
//
//     renvoyer trouves

fn compter_xmas(grille: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if grille[i][j] != 'X' { return 0; }
    let mot = ['X', 'M', 'A', 'S'];
    let direction = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut found =0;
    //         pour chaque (dl, dc) dans directions :
    //         // position de la 4e lettre
    //         l_fin <- i + 3*dl
    //         c_fin <- j + 3*dc
    //         si l_fin < 0 ou l_fin >= nombre_de_lignes
    //            ou c_fin < 0 ou c_fin >= nombre_de_colonnes :
    //             passer à la direction suivante
    for (l,c) in direction {
        let l_fin = i+3;
    }

    0
}