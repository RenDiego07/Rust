use std::{io, collections::HashMap};

const med: String = String::from("Med school");
const eng: String = String::from("engineer");
const law: String = String::from("Law school");
fn main() {
    init();
    let mut opt= String::new();
    io::stdin().read_line(&mut opt).expect("Something  went Wrong");
    let mut opt = opt.trim();
    match opt {
        "1" => {
                let mut dic:HashMap<String, Vec<String>> = HashMap::new();
                let mut flag:bool = true;
                
                set_up(&mut dic);
                while flag {
                    println!("\t\t\t 1) Show menu to add student");
                    println!("\t\t\t 2) EXIT");
                    let ans: String = get_output();
                    if ans =="2"{
                        flag = false;
                    }else if ans =="1" {
                        add_employee(&mut dic);
                    }

                }                
        },
        "2" => {

        },
    }

}
fn set_up(dic: &mut HashMap<String, Vec<String>>){
    let mut med_school_vec:Vec<String> = Vec::new();
    let mut engineer_vec:Vec<String> = Vec::new();
    let mut law_school_vec:Vec<String> = Vec::new();
    dic.insert(String::from("Med School"),med_school_vec);
    dic.insert(String::from("Law School"), law_school_vec);
    dic.insert(String::from("engineers"), engineer_vec);
}
fn add_employee(dic: &mut HashMap<String, Vec<String>>){
    let mut flag = true;
    while flag {
        println!("\t\t\tHEY WHICH DEPARTMENT DO YOU WANT TO ADD A STUDENT");
        println!("\t\t\tMed School");
        println!("\t\t\tLaw School");
        println!("\t\t\tengineers");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Something went wrong");
        let ans = ans.trim();
        
        match ans {
            "Med School"=> {
                let mut v_med = get_list(dic, med);
                add_to_list(v_med,registering());
                continue_validation(&mut flag);
            },
            "Law School"=> {
                let v_law = get_list(dic, law);
                add_to_list(v_law,registering());
                continue_validation(&mut flag);
            },
            "engineers" => {
                let v_eng = get_list(dic, eng);
                add_to_list(v_eng,registering());
                continue_validation(&mut flag);
            },
            _ => {println!("TRY AGAIN, THE DEPARTMENT GIVEN IS NON EXISTANT");
            }
        }
    }
    

}
fn registering()-> String{
    let mut student = String::new();
    io::stdin().read_line(&mut student).expect("Something went wrong");
    let student = student.trim();
    student.to_string()
}

fn delete_employee(){

}
fn init(){
    println!("\t\t\t WELCOME TO PROTO DATABASE");
    println!("\t\t\t CHOOSE ONE OF THE FOLLOWING OPTIONS");
    println!("\t\t\t 1) ADD AN EMPLOYEE");
    println!("\t\t\t 2) DELETE AN EMPLOYEE");
    println!("\t\t\t EXIT DATABASE");
}
/*fn extra_continue_validation(flag: &mut bool){
    
    continue_validation(&flag);
}
*/
fn continue_validation(flag: &mut bool){
    println!("DO YOU WANT TO ADD ANOTHER STUDENT Y/N");
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("Something went wrong");
    let mut ans = ans.trim();
    if ans == "N"{
        *flag = false;
    }
}
fn extra(){
    println!("Before you go, do u want to display in alphabetic order the student's names?");
    let mut f =  true;
    continue_validation(&mut f);
    if f{
        println!("still to be implemented");
    }
}
fn get_output()-> String{
    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("SOMETHING WENT WRONG");
    let mut ans = ans.trim(); //devuelve un &str
    ans.to_string()
}
fn get_list(map: &mut HashMap<String, Vec<String>>, area:String)->&mut Vec<String>{
    if let Some(value) = map.get_mut(&area){
        return value;
    }
    map.entry(area).or_insert(Vec::new())
}
fn add_to_list(vec :&mut Vec<String>, std: String){
    vec.push(std);
}