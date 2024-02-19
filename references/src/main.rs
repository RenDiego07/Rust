fn main() {
    let mut name = String::from("Diego Flores Rengifo");
    let len = calculate_lenght(&name);
    println!("Hey, the name {name} is {len} long");
    
    change(&mut name);
    
    // just testing
    let mut d = "hihi";
    d = "ee";
    println!("{}", d)
}

fn testi(s: &mut String) {
    *s = "eeee".to_string();
    println!("{}", s);
}

fn calculate_lenght(string: &String)-> usize {

    string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


