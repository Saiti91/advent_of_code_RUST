fn main() {
    let input = std::fs::read_to_string("inputs\\2024\\input_2024_02.txt").unwrap();
    println!("{}", resoudre(input));
}

fn resoudre(input: String) -> u32 {

    //une fonction qui teste si ma suite de nombre est strictement croissante => oui +1 => next

    //sinon une fonction qui teste si ma suite de nombre est strictement décroissante => oui +1 => next

    //sinon next
    let mut total_valid_line:u32=0;

    for line in input.lines(){
        let tab :Vec<&str> = line.split(" ").collect();
        let mut tab_num: Vec<u32> = Vec::new();

        for i in 0..tab.len() {
            tab_num.push(tab[i].parse::<u32>().unwrap())
        }
        //ici j'ai un tableau avec les nombres de ma ligne
        //Maintenant on peux tester leur ordre
        if is_croissant(&tab_num) {
            total_valid_line +=1;
        }
        else if is_decroissant(&tab_num) {
            total_valid_line +=1;
        }
        else {
            let mut skip = 0;
            for i in 0..tab_num.len(){


                let mut skip_tab:Vec<u32> = Vec::new();
                for j in 0..tab_num.len() {
                    if j!=i {
                        skip_tab.push(tab_num[j])
                    }
                }

                if is_croissant(&skip_tab) {
                    total_valid_line +=1;
                    break;
                }
                else if is_decroissant(&skip_tab) {
                    total_valid_line +=1;
                    break;
                }
            }
        }
    }
    total_valid_line
}

fn is_croissant(input:&Vec<u32>)-> bool{
    let mut croissant:bool=false;
    for i in 0..input.len()-1 {

        if input[i]<input[i+1] && input[i+1]-input[i]<=3{
            croissant=true;
        }
        else {
            return false;
        }
    }

    croissant
}


fn is_decroissant(input:&Vec<u32>)-> bool{
    let mut decroissant:bool=false;
    for i in 0..input.len()-1 {

        if input[i]>input[i+1] && input[i+1].abs_diff(input[i])<=3{
            decroissant=true;
        }
        else {
            return false
        }
    }

    decroissant
}
