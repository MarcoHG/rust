fn main() {}
#[allow(dead_code)]

pub fn sort_vector(vec_input: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec_input.to_vec();
    if new_vec.len() < 2 {
        return new_vec;
    } else {
        let mut sorted: bool = true;
        while sorted {
            sorted = false;
            for i in 1..vec_input.len() {
                if vec_input[i - 1] > vec_input[i] {
                    new_vec[i - 1] = vec_input[i];
                    new_vec[i] = vec_input[i - 1];
                    sorted = true;
                }
            }
        }
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_one_element() {
        let v = vec![1];
        assert_eq!(v, sort_vector(&v));
    }

    #[test]
    fn sort_two_elems() {
        let v = vec![2, 1];
        assert_eq!(sort_vector(&v), [1, 2]);
    }
    #[test]
    fn sort_empty_vector() {
        let v = vec![];
        assert_eq!(sort_vector(&v), []);
    }
    #[test]
    fn sort_three_elems() {
        let v = vec![2, 1, 3];
        assert_eq!(sort_vector(&v), [1, 2, 3]);
    }
    #[test]
    fn sort_three_elems_321() {
        let v = vec![3, 2, 1];
        assert_eq!(sort_vector(&v), [1, 2, 3]);
    }
}
