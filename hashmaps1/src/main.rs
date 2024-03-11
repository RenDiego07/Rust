use std::io;
use std::collections::HashMap;
// use std{io, collections::HashMap} is equal to line 1 & 2


fn main() {
    
   // println!("{}", get_output());
    let mut flag_menu:bool = true;
    let mut add_flag_menu:bool = true;
    while flag_menu{
        init();
        let ans_String = get_output();
        let ans_str = ans_String.trim();
        let mut dic: HashMap<&str, Vec<&str>> = HashMap::new();
        set_up(&mut dic);
            match ans_str{
                "1" =>{
                    while add_flag_menu{
                        add_menu(&mut dic);
                        let ans_add_m = get_output();
                        let ans_add_menu = ans_add_m.trim();
                        match ans_add_menu {
                            "1" =>{
                                println!("\t\t\t FILL THE NEXT LABELS IN ORDER TO CREATE A STUDENT");
                                let mut n1= String::new();
                                let mut n2 = String::new();
                                let mut user = make_user(&mut n1, &mut n2);
                                println!("User created {}", user);
                                show_department();
                                    let dep_output_String = get_output();
                                    let  dep_output_str = dep_output_String.trim();
                                    match dep_output_str {
                                        "1" => add_to_dic(&mut dic,dep_output_str, user),
                                        "2" => add_to_dic(&mut dic,dep_output_str, user),
                                        "3" => add_to_dic(&mut dic,dep_output_str, user),
                                        _   => {
                                            println!("ERROR, INPUT NOT VALID");
                                        }
                                    }
 
                                again(&mut add_flag_menu);
                                println!("\t\t\t GOING BACK TO MAIN MENU");
                            },
                            "2"=>{
                                println!("\t\t\tLOADING...");
                                again(&mut add_flag_menu);
                            }
                            "3"=>{
                                off_flag(&mut add_flag_menu);
                            },
                            _ =>{
                                println!("\t\t\t ERROR!! TRY PICKING A CORRECT OPTION")
                            }
                        }
                        
                    }
                        //again(&mut flag_menu);

                },
                "2" =>{
                        //again(&mut flag_menu);
                        
                },
                "3" =>{
                    off_flag(&mut flag_menu);
                    println!("\t\t\tTHX, BYE");
                    
                },
                _ =>{
                    
                    println!("\t\t\tOUTPUT NON-EXISTANT");
                    println!("\t\t\tTRY AGAIN");
                    println!("");
                    println!("");
                    
                }
            }
    }
    
}  


fn init(){
    println!("\t\t\t WELCOME TO PROTO DATABASE");
    println!("\t\t\t CHOOSE ONE OF THE FOLLOWING OPTIONS");
    println!("\t\t\t 1) ADD AN EMPLOYEE");
    println!("\t\t\t 2) DELETE AN EMPLOYEE");
    println!("\t\t\t 3) EXIT DATABASE");
}
fn get_output()-> String{
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("SOMETHING WENT WRONG ON GET_OUTPUT");
    let ans = ans.trim().to_string();
    ans

}

fn off_flag (flag: &mut bool){
    *flag = false;
}

fn set_up(dic: &mut HashMap<&str, Vec<&str>>){
    let law_school_v:Vec<&str> = Vec::new();
    let med_school_v:Vec<&str> = Vec::new();
    let engineers:Vec<&str> = Vec::new();
    dic.insert("Med School",med_school_v);
    dic.insert("law School",law_school_v);
    dic.insert("Engineers",engineers);
}

fn add_menu(dic: &mut HashMap<&str, Vec<&str>>){
    println!("\t\t\t 1) ADD STUDENT");
    println!("\t\t\t 2) SHOWCASE STUDENTS FROM EVERY DEPARTMENT");
    println!("\t\t\t RETURN TO MAIN MENU");

}
fn again(flag: &mut bool){
    println!("\t\t\tARE YOU DONE USING OUR FEATURES Y/N");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("SOMETHING WENT WRONG");
    let ans_trimmed = ans.trim();
    if ans_trimmed == "Y"{
        off_flag(flag);
    }
}

fn make_user<'a>(name: &'a mut String, name2: &'a mut String)-> &'a String{
        println!("NAME: ");
        io::stdin().read_line(name).expect("SOMETHING WENT WRONG in MAKE USER");
        println!("FIRST NAME: ");
        io::stdin().read_line(name2).expect("SOMETHING WENT WRONG in MAKE USER");
        let aux = name.trim().to_string();
        let aux1 = name2.trim().to_string();
        name.clear();
        name.push_str(&aux);     
        name.push_str(" ");   
        name.push_str(&aux1);
        name
}   

fn show_department(){
    println!("\t\t\t SELECT DEPARTMENT");
    println!("\t\t\t 1) Med School");
    println!("\t\t\t 2) Law School");
    println!("\t\t\t 3) Engineering" );

    
}

fn add_to_dic<'a>(dic: &mut HashMap<&str, Vec<&'a str>>, dep: &str, student: &'a mut str ){
   
    match  dep {
        "Med School"=>{
            if let Some(vec) = dic.get_mut("Med School") {
                // Ahora vec es &mut Vec<&str>
                vec.push(student);
                println!("{}", vec[0]);
            }
        },
        "Law School" =>{

        },
        "Engineering" =>{

        },
        _ =>{

        },

    }

}

// el problema estuvo en la funcion make_user