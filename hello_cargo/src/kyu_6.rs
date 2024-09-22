use std::cmp::Ordering;

use itertools::{enumerate, Itertools};

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

fn encrypt_this(text: &str) -> String {
    fn get_ascii(x: &str) -> String {
        x.chars().map(|x: char| format!("{}", x as u8)).join("")
    }

    fn process_word(word: &str) -> String {
        if word.len() == 1 {
            return get_ascii(word);
        } else if word.len() == 2 {
            return get_ascii(&word[..1]) + &word[1..];
        } else {
            return get_ascii(&word[..1]) + &invert_second_and_last(&word[1..]);
        }
    }

    fn invert_second_and_last(text: &str) -> String {
        let second = text.chars().take(1).join("");
        let last = text.chars().last().unwrap().to_string();
        return last + &text[1..(text.len() - 1)] + &second;
    }
    text.split(" ").map(process_word).join(" ")
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_multipliers = vec![1, 2, 3, 3, 4, 10];
    let evil_multipliers = vec![1, 2, 2, 2, 3, 5, 10];

    fn calculate_power(army: &str, multiplier: Vec<u32>) -> u32 {
        army.split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .zip(multiplier)
            .map(|x| x.0 * x.1)
            .sum()
    }
    let good_power = calculate_power(good, good_multipliers);
    let evil_power = calculate_power(evil, evil_multipliers);

    match good_power.cmp(&evil_power) {
        Ordering::Greater => "Battle Result: Good triumphs over Evil".to_string(),
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good".to_string(),
        Ordering::Equal => "Battle Result: No victor on this battle field".to_string(),
    }
}

fn make_readable(seconds: u32) -> String {
    let minutes = &seconds % 3600 / 60;
    let remaining = &seconds % 60;
    format!("{:0>2}:{:0>2}:{:0>2}", &seconds / 3600, minutes, remaining)
}
fn ips_between(start: &str, end: &str) -> u32 {
    fn parse_ip(ip: &str) -> Vec<i64> {
        ip.split(".")
            .map(|num| num.parse::<i64>().unwrap())
            .collect()
    }
    let ip_differences = &parse_ip(end)
        .iter()
        .zip(parse_ip(start).iter())
        .map(|x| x.0 - x.1)
        .rev()
        .collect::<Vec<_>>();

    let mut result: i64 = 0;
    for (idx, diff) in enumerate(ip_differences) {
        let modifier = i64::pow(256, idx as u32) * diff;
        result += modifier;
    }
    result as u32
}

fn min_umbrellas(weather: &[&str]) -> usize {
    let mut home_stock = 0;
    let mut work_stock = 0;
    for (time, &weather) in weather.iter().enumerate() {
        match weather {
            "rainy" | "thunderstorms" => {
                if time % 2 == 0 {
                    work_stock += 1;
                    if home_stock > 0 {
                        home_stock -= 1;
                    }
                } else {
                    home_stock += 1;
                    if work_stock > 0 {
                        work_stock -= 1;
                    }
                }
            }
            _ => (),
        };
    }
    home_stock + work_stock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests_umbrella() {
        assert_eq!(min_umbrellas(&["cloudy"]), 0);
        assert_eq!(min_umbrellas(&["rainy", "rainy", "rainy", "rainy"]), 1);
    }
    #[test]
    fn sample_tests_umbrella_difficult() {
        assert_eq!(
            min_umbrellas(&["rainy", "rainy", "rainy", "rainy", "thunderstorms", "rainy"]),
            1
        );
    }
    #[test]
    fn sample_tests_umbrella2() {
        assert_eq!(
            min_umbrellas(&["overcast", "rainy", "clear", "thunderstorms"]),
            2
        );
        assert_eq!(
            min_umbrellas(&["overcast", "overcast", "rainy", "clear", "thunderstorms"]),
            2
        );
        assert_eq!(
            min_umbrellas(&["rainy", "overcast", "rainy", "clear", "thunderstorms"]),
            3
        );
    }

    #[test]
    fn test_ip_adress() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
    #[test]
    fn fixed_tests() {
        assert_eq!(make_readable(0), "00:00:00");
        assert_eq!(make_readable(59), "00:00:59");
        assert_eq!(make_readable(60), "00:01:00");
        assert_eq!(make_readable(3599), "00:59:59");
        assert_eq!(make_readable(3600), "01:00:00");
        assert_eq!(make_readable(86399), "23:59:59");
        assert_eq!(make_readable(86400), "24:00:00");
        assert_eq!(make_readable(359999), "99:59:59");
    }
    fn do_test(good: &str, evil: &str, expected: &str) {
        let actual = good_vs_evil(good, evil);
        assert_eq!(
        actual, expected,
        "\n  Good: \"{good}\n  Evil: \"{evil}\"\nYour answer (left) is not the expected answer (right).",
    );
    }

    #[test]
    fn test_good_wins() {
        do_test(
            "1 0 1 0 0 0",
            "1 0 0 0 0 0 0",
            "Battle Result: Good triumphs over Evil",
        );
        do_test(
            "0 0 0 0 0 10",
            "0 0 0 0 0 0 0",
            "Battle Result: Good triumphs over Evil",
        );
    }
    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(encrypt_this(&"in"), "105n".to_string());
        assert_eq!(encrypt_this(&"an"), "97n".to_string());
        assert_eq!(encrypt_this(&"Hello"), "72olle".to_string());
        assert_eq!(encrypt_this(&"good"), "103doo".to_string());
        assert_eq!(
            encrypt_this(&"A wise old owl lived in an oak"),
            "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string()
        );
        assert_eq!(
            encrypt_this(&"The more he saw the less he spoke"),
            "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string()
        );
        assert_eq!(
            encrypt_this(&"The less he spoke the more he heard"),
            "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string()
        );
        assert_eq!(
            encrypt_this(&"Why can we not all be like that wise old bird"),
            "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string()
        );
        assert_eq!(
            encrypt_this(&"Thank you Piotr for all your help"),
            "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string()
        );
    }
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
