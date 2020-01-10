fn main() {
	println!("Hello World");
}
	
#[allow(dead_code)]

fn roman_add( lhs: &str, rhs: &str) -> String {
	let left 	= String::from(lhs).replace("IV","IIII").replace("V","IIIII");
	let right = String::from(rhs).replace("IV","IIII").replace("V","IIIII");

	let s = left + &right;
	let r = s.replace("IIIII","V").replace("IIII","IV").replace("VV","X");
	println!("Hello {}",r);
	r
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

	#[test]
	fn test_add_ii_i() {
		assert_eq!(roman_add("II","I"), "III");
	}

	#[test]
	fn test_add_i_iii() {
		assert_eq!(roman_add("I","III"), "IV");
	}

	#[test]
	fn test_add_ii_ii() {
		assert_eq!(roman_add("II","II"), "IV");
	}

	#[test]
	fn test_add_i_iv() {
		assert_eq!(roman_add("I","IV"), "V");
	}
	
	#[test]
	fn test_add_iv_i() {
		assert_eq!(roman_add("IV","I"), "V");
	}

	#[test]
	fn test_add_ii_iii() {
		assert_eq!(roman_add("II","III"), "V");
	}

	#[test]
	fn test_add_iii_ii() {
		assert_eq!(roman_add("III","II"), "V");
	}
	
//	#[test]
//	fn test_add_iv_v() {
//		assert_eq!(roman_add("IV","V"), "IX");
//	}
	
	
}

