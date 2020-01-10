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
	(convert(lhs) + &convert(rhs)).replace("IIIII","V").replace("IIII","IV").replace("VV","X")
}
