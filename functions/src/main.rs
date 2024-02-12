fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len); // no se puede usar el s1 porque debido al 
                                                    // ownership cada variable debe de tener solo 
                                                    // un dueño
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}