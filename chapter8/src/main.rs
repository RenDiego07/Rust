use std::{arch::global_asm, collections::HashMap};

enum team{
    Green(u32),
    Blue(u32),
}
impl team {
    fn get_score(&self) -> u32{
        match self {
            team::Green(value)=> *value,
            team::Blue(value)=> *value,
        }
    }
}

fn main() {

    let mut v: Vec<i32>  =  Vec::new();
    // otra forma de escribir un vector
    let mut v1 = vec![2,23,2,2];
    v1.push(3);
    
    let third: i32 = v1[2];

    for element in v1 {
        println!("{element}");
    }

    let mut v2 = vec!["J'm", "apelle", "DR"];


    let w = v2.get(100);
    if let none = w {
        println!("WORKED");
    }
  let mut stack:Vec<String> = Vec::with_capacity(5);


  let mut n1 = "Diego";
  let mut n2 =  "Flores";
  let mut n3 = "Rengifo";

    let full_name= format!("{n1}- {n2} - {n3}");
    println!("{full_name}");
    


    // HASHMAPS
   /*  let mut scoring_table= HashMap::new();
        let mut green: team = team::Green(10);
        let mut blue: team = team::Blue(50);
        scoring_table.insert(green,green.get_score());
        
            me indicaba que team necesitaba un trait
        */

    let mut scoring_table = HashMap::new();
    scoring_table.insert(String::from("Green"), 10);
    scoring_table.insert(String::from("Blue"), 50);
    
    let blue_score= scoring_table.get(&String::from("Blue")).copied().unwrap_or(0);

    println!("{blue_score}");

    for (key, value) in &scoring_table{
        println!("{key}, {value}");

    }

    let text = "hello world";

    let mut vector = vec![1,2,5,3,8,9,10,11];

   /*  vector.sort();
    let length = vector.len() / 2;
    if let Some(some) = vector.get(length){
        println!("{}", some);
    }
    */
    sort(&mut vector);
    for index in 0.. vector.len()-1{
        println!("{}", vector[index]);
    }
    
    

    

    


}
/* 
fn sort(vec:&mut Vec<i32>)-> &Vec<i32>{
    
    let len = vec.len() as i32;
    let mut timer = 0;
    while timer <= len {
        
        for (index, element) in vec.iter().enumerate(){
            let next: Option<i32> = vec.get(index+1).copied();
            if let Some(value) = next {
                if *element > value {
                    swap(index as i32, (index+1) as i32);
                }
            }
        
        }
        timer = timer+1;
    }
    vec
    
}
*/

fn sort(vec: &mut Vec<i32>)-> &Vec<i32>{
    
    let mut times:i32 = 0;
    let V_size = vec.len() as i32;
    
    while(times< V_size ){
        for index in 0.. V_size-1 {
            let next =  vec.get((index+1) as usize);
            if let Some(R) = next{
                if vec[index as usize]> *R {
                    swap(vec, index as usize, (index + 1) as usize);
                }
            }
        }
        times = times+1;
    }
    vec

}


fn swap(vec :&mut Vec<i32>, ind1: usize, ind2: usize){
    let aux = vec[ind2];
    vec[ind2] = vec[ind1];
    vec[ind1] = aux; 
}