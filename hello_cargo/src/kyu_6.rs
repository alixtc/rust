use itertools::Itertools;

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

fn find_uniq(arr: &[f64]) -> f64 {
    let first = &arr[0];
    let filtered = arr.iter().filter(|x| x != &first).collect::<Vec<_>>();
    if filtered.len() == 1 {
        return filtered[0].to_owned();
    } else {
        return first.to_owned();
    }
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let int_sqrt = |x: u64| (x as f64).sqrt() as u64;
    let get_divisors = |x: u64| (1..x + 1).filter(move |y| (x % y) == 0);

    let mut results = Vec::new();
    for i in m..n {
        let divisors = get_divisors(i).collect::<Vec<_>>();
        let sq_div_sum = divisors.iter().map(|x| x * x).sum();
        if int_sqrt(sq_div_sum) * int_sqrt(sq_div_sum) == sq_div_sum {
            results.push((i, sq_div_sum));
        }
    }
    results
}

fn order(sentence: &str) -> String {
    if sentence.is_empty() {
        return String::from("");
    }
    let extract_number = |x: &str| x.chars().find(|y| y.is_digit(10));
    sentence
        .split(" ")
        .map(|x| (x, extract_number(x)))
        .sorted_by_key(|x| x.1)
        .map(|x| x.0)
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }

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

    #[test]
    fn examples() {
        assert_eq!(find_uniq(&[0.0, 1.0, 0.0]), 1.0);
        assert_eq!(find_uniq(&[1.0, 1.0, 1.0, 2.0, 1.0, 1.0]), 2.0);
        assert_eq!(find_uniq(&[3.0, 10.0, 3.0, 3.0, 3.0]), 10.0);
    }

    fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(list_squared(m, n), exp)
    }

    #[test]
    fn basics_list_squared() {
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(42, 250, vec![(42, 2500), (246, 84100)]);
        testing(300, 600, vec![]);
    }
}
