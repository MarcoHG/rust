fn main() {
	println!("{}, III", roman_add( "I","II") );
	println!("{}, X", roman_add("VI","IV"));	// "X");
	println!("{}, IX", roman_add("IV","V"));	// "IX");

}



#[allow(dead_code)]

fn convert(s: &str) ->String {
	String::from(s).replace("IV","IIII").replace("V","IIIII")
}

fn roman_add( lhs: &str, rhs: &str) -> String {
	(convert(lhs) + &convert(rhs)).replace("IIIII","V").replace("IIII","IV").replace("VIV","IX").replace("VV","X")
}


#[cfg(test)]
// cargo test
mod tests {
	use super::*;
	
	#[test]
	fn test_add_i_i() {
		assert_eq!(roman_add("I","I"), "II");
	}

	#[test]
	fn test_add_vi_iv() {
		assert_eq!(roman_add("VI","IV"), "X");
	}

	#[test]
	fn test_add_iv_vi() {
		assert_eq!(roman_add("IV","VI"), "X");
	}

	#[test]
	fn test_add_iv_v() {
		assert_eq!(roman_add("IV","V"), "IX");
	}
}
