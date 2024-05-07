use std::{env, fs, process};
use minigrep::{brightness, instructions, config_arg, Config};

fn main() {
    let v_args: Vec<String> =env::args().collect();
    println!("{}", brightness::decay());
    //dbg!(&arg[2]);

    /*let mut file_path = &v_args[2];
    println!("{}", file_path);
    let content = fs::read_to_string(file_path).expect("tuvo que haber funcionado");
    println!("\n{content}");
    */
        // in order to compile successfully it's neccesary to parse the next argument "cargo run -- f poem.txt" 
   let parse_config:Result<Config, &str> = minigrep::Config::build(&v_args);
   
        // changes made  changed method new to build
//    let content = fs::read_to_string(parse_config.path).expect("it should've worked");



        let content = match parse_config {
            Ok(Co) =>  fs::read_to_string(Co.path).expect("it should've worked"),
            Err(strf)  => String::from("hihi"),
        };

        
        // debido a que en arm del match se devolvia un valor entonces por consecuencia tambien lo debia de hacer el otro
        // no lo hizo -> no funcionaba 

        // sugar code

        /*/  let parse_config:Result<Config, &str> = minigrep::Config::build(&v_args).unwrap_or_else(│err│{
                println!("problem parsing arguments! {err}");
                process::exit(1);
        });
         */



    println!("\n{content}");
     

}
