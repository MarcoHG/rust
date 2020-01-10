fn main() {
	println!("Hello World");
}
	
#[allow(dead_code)]
fn roman_add( lhs: &str, rhs: &str) -> String {
	let left = String::from(lhs).replace("IV","IIII")
		.replace("V","IIIII");
	let right = String::from(rhs).replace("IV","IIII")
		.replace("V","IIIII");
	let s = left + &right;
	s.replace("IIIII","V").replace("IIII","IV")
		.replace("VV","X")
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_add_i_i() {
		assert_eq!(roman_add("I","I"), "II");
	}	

	#[test]
	fn test_add_i_ii() {
		assert_eq!(roman_add("I","II"), "III");
	}	
}

