use Remember::{Student, Ip,Summary, NewsArticle};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};

fn get_number(string:&str)->Result<i32,io::Error>{
    let mut us_response = String::new();
    io::stdin().read_line(&mut us_response)?;
    let number:i32 = match us_response.parse() {
        Ok(num) => num,
        Err(err) => panic!("gg"),
    };
    Ok(number)


}




fn get_number1() ->Result<i32, io::Error> {
    let mut str = String::new();
    io::stdin().read_line(&mut str)?;
    let number:i32 = match str.parse(){
        Ok(num) => num,
        Err(err) => panic!("ggs"),
    };
    Ok(number)
}
fn test(str: &str, str1: &str){

}


fn main() {


    let array: [i32; 4] = [1,2,3,4];
    
    for index in 0..array.len(){
        println!("{}", index);
    }

    let de: Option<i32>  = Some(3);
    match de {
        Some(x) => {
            println!("{}", x);
        },
        None => {
            println!("Nothing");
        },
    }
    
    let mut reff: String = String::from("hihi");
    
    let slice = &reff[0..2];
    
    let mut str3 : String = "Diego".to_string();
    let stud = Student::new("Diego", "Flores", 20);
    println!("{}", stud.get_name());


    let ip = Ip::ipV4(String::from("1298498"));
    match ip {
        Ip::ipV4(x) => {
            println!("{}", x);
        }
        Ip::ipV6(y)=> {
            println!("{}", y);
        }

    }


    let op: Option<String> = Some(String::from("HIHI"));
    op.unwrap_or(String::from("Diego"));

    println!("{:?}", stud);

    // chapter 8 


    let mut v1 = Vec::new();
    v1.push("hihi");
    
    
    let array: [i32; 4] = [2,3,41,4];
    for &numbers in array.iter(){
        println!("{}", numbers);
    }

    //Recall the rule that states you can’t have mutable and immutable references in the same scope
    let mut numb = 10;
    let mutable = &mut numb;
    //let not_mutable = &numb;  enfasizes in the text above
    *mutable = 5;
    println!("{}", numb);

    let mut ar : [u32; 5] = [0,1,2,3,4];
    for element in &mut ar{
        println!("{}", element);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // es obligacion pasar una referencia como argumento en get 



    let mut map1 = HashMap::new();
    map1.entry(String::from("Ecuador")).or_insert(20);  // en caso de que exista este devuelve una referencia al value de la key
    map1.entry(String::from("Mexico")).or_insert(30);

    for (key, value) in &map1 {
            println!("{}  {}", key, value);
    }
    
    let phrase = String::from("j'adore la vie");
    let mut dic2  = HashMap::new();
    for word in phrase[..].split_whitespace(){
        let mut count = dic2.entry(word).or_insert(0);
        *count += 1;
    }

    //println!("{:?}", dic2);
    //panic!("CRASH DOWN");
    let file = File::open("path");
    let file = match file {
        Ok(s) => s,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("PATH"){
                Ok(fi)=> fi,
                Err(error) => panic!("gg")
            }
            other_error=>{
                panic!("ggs")
            }
        }
    };

    // chapter 10
    let n_A = NewsArticle{
        headline: String::from("dd"),
        location: String::from("ee"),
        author: String::from("qq"),
        content: String::from("ww"),
    };
    println!("{}", n_A.summarize());




}


