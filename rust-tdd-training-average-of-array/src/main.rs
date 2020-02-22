fn main() {}
#[allow(dead_code)]
// returns average of a array of floats.
// the input array 'a', a: &[f32] is passed in by referance.
// the function returns the avg of all the values as type 32bit float  "-> f32" 
fn get_average(a: &[f32]) -> f32 {
    // if the array is empty then return 0.
    if a.len() == 0 {
        return 0.0;
    }
    else
    {
        // call the sum_f32 function and pass by referance the array 'a[]'
        // then divide the sum by the length of the array recasted as a float 32 and return the 
        // avg value as float 32.
        sum_f32(&a) / a.len() as f32
    }
}

fn sum_f32(a: &[f32]) -> f32 {
    let mut sum = 0.0;
    for element in a.iter() {
        sum = sum + element;
    }
    return sum;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_average_of_zeros_return_zero() {
        let a = [0.0, 0.0];
        assert_eq!(0.0, get_average(&a));
    }
    #[test]
    fn test_average_of_zero_and_one_return_p5() {
        let a = [1.0, 0.0];
        assert_eq!(0.5, get_average(&a));
    }
    #[test]
    fn test_average_of_empty_array() {
        let a = [];
        assert_eq!(0.0, get_average(&a));
    }
    #[test]
    fn test_average_of_0_1_2() {
        let a = [0.0, 1.0, 2.0];
        assert_eq!(1.0, get_average(&a));
    }
    #[test]
    fn test_average_of_large_number_and_large_negnumber() {
        let big_number = std::f32::MAX;
        let a = [0.0, 1.0 * big_number, -1.0 * big_number];
        assert_eq!(0.0 * big_number, get_average(&a));
    }

    #[test]
    fn test_average_of_over_flow() {
        let big_number = std::f32::MAX;
        let a = [0.0, 1.0 * big_number, 1.0 * big_number];
        assert_eq!(std::f32::INFINITY, get_average(&a));
    }
}
