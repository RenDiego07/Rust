use std::{fmt::Display, iter::Sum};


#[derive(Debug)]
pub struct Student<'a> {
    pub name: &'a str,
    pub first_name : &'a str,
    pub age : u32,

}
pub enum Ip {
    ipV4(String),
    ipV6(String),

}
impl Student<'_>{

    pub fn new<'a>(name: &'a str, first_name: &'a str, age: u32)-> Student<'a>{
        Student {
            name,
            first_name,
            age
        }
    }  
    pub fn get_name(&self)-> &str{
        self.name
    }
}

//chapter 10 traits

pub trait Summary{

    fn summarize(&self)-> String;
}




pub struct NewsArticle {

    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self)-> String{
        format!("{} {}", self.headline, self.author)
    }

}
// traits as Parameters
pub fn notify<T: Summary>( su: &T, su1: &T){

}

pub fn notify2<'a>(su: &'a impl Summary, su2: &'a impl Summary ){

}

pub fn report(su: &(impl Summary +  Display)){
    
}

pub fn report1<T: Summary+ Display>(su: &T){
}


// WHERE CLAUSE


pub fn get_total<T, A>( value: &T, value1 :&A) -> i32
    where 
        T: Display,
        A: Summary,
        {
            43
        }

pub fn report2<T,U >(su: &T, su1: &U) -> i32
    where 
        T: Summary+Display,
        U: Summary + Clone,
        {
                32
        }

