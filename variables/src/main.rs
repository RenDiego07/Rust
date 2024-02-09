//const n2um: u32 = 64;

use std::collections::{linked_list, LinkedList};


fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    {
        let x = x*5;
        println!("el valor modifaco dentro de un inner scope es {x}");

    }
    println!("Fuera de este es {x}");
    let name:&str= "Diego";
    let nam = name.len();
    println!("cantidad {name}");

    let Dlis: [&str; 2] = ["Diego", "Flores"];


}