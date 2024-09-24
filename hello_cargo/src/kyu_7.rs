use itertools::Itertools;

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    numbers.iter().sorted().take(2).sum()
}

fn get_middle(data: &str) -> &str {
    let middle_point = data.len() / 2;
    if (data.len() % 2) != 0 {
        &data[middle_point..middle_point + 1]
    } else {
        &data[middle_point - 1..middle_point + 1]
    }
}
fn hello(name: &str) -> String {
    if name.is_empty() {
        return String::from("Hello, World!");
    };
    format!(
        "Hello, {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_with_diffent_minimums() {
        assert_eq!(sum_two_smallest_numbers(&[5, 8, 12, 19, 22]), 13);
        assert_eq!(sum_two_smallest_numbers(&[15, 28, 4, 2, 43]), 6);
        assert_eq!(sum_two_smallest_numbers(&[23, 71, 33, 82, 1]), 24);
        assert_eq!(sum_two_smallest_numbers(&[52, 76, 14, 12, 4]), 16);
    }
    #[test]
    fn sample_tests_with_same_minimums() {
        assert_eq!(sum_two_smallest_numbers(&[1, 1, 5, 5]), 2);
    }
    #[test]
    fn test_get_middle() {
        assert_eq!(get_middle("test"), "es");
        assert_eq!(get_middle("testing"), "t");
        assert_eq!(get_middle("middle"), "dd");
        assert_eq!(get_middle("A"), "A");
        assert_eq!(get_middle("of"), "of");
    }

    #[test]
    fn test_hello() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
