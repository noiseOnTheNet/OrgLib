
mod org;
use org::{Node, Status};

#[cfg(test)]
mod tests {
	use std::fs::File;
	use std::io::{self, BufRead};
	use std::path::Path;
	// The output is wrapped in a Result to allow matching on errors
	// Returns an Iterator to the Reader of the lines of the file.
	fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where
		P: AsRef<Path>,
	{
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}

	#[test]
	fn prova() {
		let mut lines_count: usize = 0;
		if let Ok(lines) = read_lines("./test1.org") {
			// Consumes the iterator, returns an (Optional) String
			for line in lines {
				if let Ok(_ip) = line {
					lines_count += 1;
				}
			}
		}
		crate::main();
		assert!(lines_count > 0);
	}

}

fn main() {
	let mut x = Node::new(1, "hi".to_string());
	let mut y = Node::new(2, "there".to_string());
	let z = Node::new(2, "how".to_string());
	let w = Node::new(3, "are".to_string());
	y.add_child(w).unwrap();
	x.add_child(y).unwrap();
	x.add_child(z).unwrap();
	x.status = Some(Status::TODO);
	println!("Debug {:?}", x);
	println!("Display {}", x);
}
