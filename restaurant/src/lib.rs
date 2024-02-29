use crate::front_of_house::hosting;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}



mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
        
    }
}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order(); // a través de super podemos referenciar al modulo padre del modulo donde estamos
                                //estamos, en este caso el crate. De esta manera no tenemos que poner
                                // el path relativo o no relativo 
    }
    fn cook_order(){}

}
mod customer {
    use super::hosting; // o tambien funciona  crate::front_of_house::hosting

    pub fn eat_at_restaurant(){
        hosting::add_to_waitlist();
    }
}
pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}

fn deliver_order(){
}

