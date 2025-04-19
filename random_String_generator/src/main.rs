use rand::prelude::*;

fn generate_random() -> Option<u8> {
    let mut rng = rand::rng();
    // Generate and shuffle a sequence:
    let mut nums: Vec<u8> = (33..126).collect();
    nums.shuffle(&mut rng);
    // And take a random pick (yes, we didn't need to shuffle first!):
    
    let result= *(nums.choose(&mut rng).unwrap());
    
    Some(result)
}

fn main(){
    
    let mut result= String::new();
    let mut idx= 0;
    while idx < 1000 {
        let one_random_integer= generate_random();
        let c= one_random_integer.unwrap() as char;

        result.push_str(&format!("{}", c));
        
        idx+=1;
    }

    println!("{}", result);
}