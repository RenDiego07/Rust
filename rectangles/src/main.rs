#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
struct triangule{
    width: u32,
    height: u32,
}
impl triangule {
    fn area(&mut self)-> u32{
        (self.width*self.height)/2
    }
    fn height(&mut self) -> u32{
        self.height
    }
    fn get_width(&mut self)-> u32 {
        self.width
    }
    fn can_hold(&mut self, other: &triangule)-> bool{
        self.height> other.height && self.width > other.width
    }
    fn default_size()->Self{
        Self{
            width:10,
            height:10
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    let /*  not having mut causes an error*/ mut t1 = triangule{
        width: 2,
        height:8,
    };

    println!("rect1 is {:#?}", rect1);
    println!("area equals to {}", area(&rect1));
    println!("area of triangule is {}", t1.area());
    println!("{}", t1.get_width());

    let mut d: triangule = triangule::default_size();
    println!("{}",d.get_width());

}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}