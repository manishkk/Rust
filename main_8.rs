fn main() {
		//code block
		let x = 10;

		{
			let y = 5;
			println!("x: {} y: {}", x, y);
		}

	// println!("x: {} y: {}", x, y);  can't find value of y because scope of y is local in block only
}
