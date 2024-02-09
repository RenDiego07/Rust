
use std::io;
use rand::Rng;


fn game(){
    let mut count : u32 = 0;
    
    let ans: String = user_response();
    if ans.is_empty(){

        println!("Have a nice day!!");
    }else{  
        let num_to_guessed= user_Num();
        match num_to_guessed {
            Ok(number) => {
                loop{
                    let ran_num = rand::thread_rng().gen_range(1..50);
                        if ran_num == number{
                            println!("{} tries", count);
                            break;
                        }else{
                            count= count+1;
                        }
                    }
                }
            Err(error) => {
                    println!("ERROR {}", error);
                }
            } 
        }
    }
 

fn user_response()-> String{

    println!("U WANNA HOW LUCKY U'RE:");
    let mut ans= String::new();
    io::stdin().read_line(&mut ans).expect("SOMETHING WENT WRONG");
    if ans == "Yes"{
        ans.trim().to_string()
    }else{
        String::new()
    }
}
fn user_Num() -> Result<u32, String>{
    println!("Hi, enter any number >> ");
    let mut number = String::new();
    io::stdin()
    .read_line(&mut number)
    .expect("SOMETHIN WENT WRONG");
    let guessed_number = number.trim();
     match number.parse(){
        Ok(num) => Ok(num),
        Err(_) => Err(format!("SORRY {} IS NOT CONSIDERED A NUMBER", guessed_number)),
    }
}



fn main() {
    game();
}
