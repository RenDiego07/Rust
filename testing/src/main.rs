//use core::num;
use std::{io, process::ExitCode};

fn main() {
	println!("\t\ttenemos el siguiente arreglo");
	
	let mut x:usize = 0;
	let d_List: [i32; 5] = [2,3,5,1,7];
	let size:usize = d_List.len();
	
	let mut selected: String = String::new();
	
	print!("\t[");
	loop{
		print!("\t{}",d_List[x]);		
		if x +1 == size {
				break;
		}
		x = x+1;
	}
	println!("\t]");
	println!("Escoga un numero");

	io::stdin()
	.read_line(&mut selected)
	.expect("failed to read line");

	let selected: usize = match selected.trim().parse(){
		Ok(num) => num,
		Err(_) => {
			println!("g");
			0
			
		}
	};
	println!("escogio el numero {} que se encuentra en el indice {}",d_List[selected],selected);
}
