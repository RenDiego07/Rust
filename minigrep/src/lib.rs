
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