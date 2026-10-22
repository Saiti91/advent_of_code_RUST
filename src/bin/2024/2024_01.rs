fn main() {
    let input = std::fs::read_to_string("inputs\\2024\\input_2024_01.txt").unwrap();
    println!("{}", resoudre(input));
}

fn resoudre(input: String) -> u64 {
    let mut left_list : Vec<u32> = Vec::new();
    let mut right_list : Vec<u32> = Vec::new();
    for line in input.lines(){
        let mut morceaux : Vec<&str> = Vec::new();
        morceaux=line.split_whitespace().collect();
        left_list.push(morceaux[0].parse::<u32>().unwrap());
        right_list.push(morceaux[1].parse::<u32>().unwrap());
    }
    tri(&mut left_list);
    tri(&mut right_list);

    mult_same(left_list,right_list)

}

fn tri(input:&mut Vec<u32>){
    let mut exchange:bool = true;
    while exchange == true {
        exchange = false;
        for i in 0..input.len()-1 {
            if input[i]>input[i+1]{
                let temp = input[i];
                input[i] = input[i+1];
                input[i+1] = temp;
                exchange = true;
            }
        }
    }
}

fn add_distance(left_list:Vec<u32>, right_list:Vec<u32>)-> u32{
    let mut total : u32 = 0;
    for i in 0..left_list.len() {
        total+= left_list[i].abs_diff(right_list[i]);
    }
    total
}

fn mult_same(left_list:Vec<u32>, right_list:Vec<u32>)->u64{

    let mut total_appearance : u32 = 0;
    let mut total : u64 = 0;
    for i in 0..left_list.len() {
        for j in 0..right_list.len() {
            if left_list[i] == right_list[j]{
                total_appearance+=1;
            }
        }
        total += (left_list[i]*total_appearance) as u64;
        total_appearance = 0;
    }
    total
}