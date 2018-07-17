fn main_10(){

	//referneces
	let mut x = 10;
	//let xr = &x; //get referneces

	{
		let dom = &mut x; //mutable referneces
		*dom += 1;
	}

	
	println!("x is {}", x);
	//println!("x is {}", dom);
}
