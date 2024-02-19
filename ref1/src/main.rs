fn main() {
    let mut name = String::from("Diego Flores");
    let n1 = second_word(&name);
    println!("my first name is {}", n1);
   // name.clear(); compile error
    //println!("{}", n1);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn second_word(s:&String)-> &str{
    
    let s_bytes = s.as_bytes();
    let mut index: usize = 0;
    let mut times: i32 = 0;
    for(x, &item) in s_bytes.iter().enumerate(){
        if times ==1 {    
            if item == b' '{
                return &s[index..x];
            }   
        }else{
            if item == b' '{
                times= times+1;
                index  = x;   
            }
        }
    }
    if times ==1 {
       return &s[index..];
    }
    &s[..]

}




/*fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}*/
