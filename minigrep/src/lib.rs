use std::error::Error;
use std::{env, fs, process};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ductive";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut v_rt = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            v_rt.push(line);
        }

    }
    v_rt
}
pub struct Config<'a>{
    pub path: &'a String,
    pub query: &'a String,
}
pub fn config_arg(vec: &Vec<String>)-> Config{
    Config {
        path: &vec[2],
        query: &vec[1],
        
    }

}
impl<'a > Config <'a>{
    // antes tenia un self, pero eso solo es para acceder a datos de la estructura
    pub fn build(vec:&Vec<String>)-> Result<Config, &'static str>{
        if vec.len() ==3 {
        return  Ok( Config {
            query :  &vec[1],
            path : &vec[2],
        })
      }
      Err("not enough arguments")

    }
}
pub fn instructions(){
    println!("INSTRUCTIONS, FOLLOW THEM");
}
pub mod brightness{

    pub fn decay()-> String{
        String::from("hihi")
    }
    
}

pub fn run (con: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(con.path)?;
    for lines in search(&con.query, &contents){
        println!("{lines}");   // a diferencia a de println!("{}", lines) esta imprimira el texto y no el valor
    }
    Ok(())

}