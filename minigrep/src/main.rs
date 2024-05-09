use std::{env, fs, process};
use minigrep::{brightness, instructions, config_arg, Config};
// gibran alcocer 
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
   let parse_config=Config::build(&v_args).unwrap_or_else(|err|{ // no se puede poner Result<Config, &'static String> porque el unwrap te da el valor de Ok() no Result 
        println!("Problem parsing arguments {err}");    // como dice la funcion unwrap_or_else las llaves estan con el proposito
        process::exit(1);                               // de manejar el caso de que sea un Err en Result
   });

   // manejo la posibilidad de que no tengan los suficiente argumentos
   minigrep::run(parse_config);
   
   
        // changes made  changed method new to build
//    let content = fs::read_to_string(parse_config.path).expect("it should've worked");


    /* 
        let content = match parse_config {
            Ok(Co) =>  fs::read_to_string(Co.path).expect("it should've worked"),
            Err(strf)  => String::from("hihi"),
        };

       */ 
        // debido a que en arm del match se devolvia un valor entonces por consecuencia tambien lo debia de hacer el otro
        // no lo hizo -> no funcionaba 

        // sugar code

        /*/  let parse_config:Result<Config, &str> = minigrep::Config::build(&v_args).unwrap_or_else(│err│{
                println!("problem parsing arguments! {err}");
                process::exit(1);
        });
         */



         //learned
         /*
            distintas formas para modularizar codigo cuando se puede obtener errores
            modularizar con Result
            Result<(), Box<dyn Error>>
            librerias std::fs::process(para retornar un numero distinto de 0 como error)
                      std::env (para usar funcion args() y luego collect())
                      args() devuelve un iterador que produce argumentos pasados a traves del shell
                      collect() convierte este iterador en un vector 
          */

          
}
