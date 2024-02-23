//ENUMS
//Option<T>
enum IpAddrKind{
    v4,
    v6,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/*impl Message{
    fn call(&self){
        println!("{}", self.)
    }
}
*/
struct server{
    IpAdrr: IpAddrKind,
    some: u32,
}
fn print_addr(add: IpAddrKind){
    
}

fn main() {
   let four = IpAddrKind:: v4; 
    let s: Option<String> = Some(String::from("XD"));
    match s {
        Some(value)=>{
                println!("hihi");
        }
        None => {
                println!("None");
        }
    }
    let m = Message::Write(String::from("Diego"));
    match m {
        Message::Write(s) => {
            println!("U were right");
        }
        Message::Quit => {
            println!("QUIT");
        }
        Message::Move{x,y} =>{
            println!("Move");
        }
        Message::ChangeColor(color1,color2,color3)=>{
            println!("ChangeColor");
        }
    }
}



