fn main() {
    let name = String::from("Diego Flores Rengifo");
    let len = calculate_lenght(&name);
    println!("Hey, the name {name} is {len} long");
    
    
    
    // just testing
    let mut d = "hihi";
    d = "ee";
    println!("{}", d)
}


fn calculate_lenght(string: &String)-> usize {

    string.len()
}
