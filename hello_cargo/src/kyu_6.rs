fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut array: Vec<u8> = Vec::new();
    for val in lst {
        let number_of_elements = array.iter().filter(|x| x == &val).count();
        if number_of_elements < n {
            array.push(val.to_owned());
        }
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_delete_nth() {
        assert_eq!(delete_nth(&[20, 37, 20, 21], 1), vec![20, 37, 21]);
        assert_eq!(
            delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3),
            vec![1, 1, 3, 3, 7, 2, 2, 2]
        );
        assert_eq!(
            delete_nth(&[1, 2, 3, 1, 2, 1, 2, 3], 2),
            vec![1, 2, 3, 1, 2, 3]
        )
    }
}
