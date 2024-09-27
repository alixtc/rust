use counter::Counter;
use std::cmp::{min, Ordering};

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
        filtered[0].to_owned()
    } else {
        first.to_owned()
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
    let extract_number = |x: &str| x.chars().find(|y| y.is_ascii_digit());
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
            get_ascii(word)
        } else if word.len() == 2 {
            get_ascii(&word[..1]) + &word[1..]
        } else {
            get_ascii(&word[..1]) + &invert_second_and_last(&word[1..])
        }
    }

    fn invert_second_and_last(text: &str) -> String {
        let second = text.chars().take(1).join("");
        let last = text.chars().last().unwrap().to_string();
        last + &text[1..(text.len() - 1)] + &second
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
    let mut home_stock: u32 = 0;
    let mut work_stock: u32 = 0;
    for (time, &weather) in weather.iter().enumerate() {
        match weather {
            "rainy" | "thunderstorms" => {
                if time % 2 == 0 {
                    work_stock += 1;
                    home_stock = home_stock.saturating_sub(1);
                } else {
                    home_stock += 1;
                    work_stock = work_stock.saturating_sub(1);
                }
            }
            _ => (),
        };
    }
    (home_stock + work_stock) as usize
}

fn get_pins(observed: &str) -> Vec<String> {
    fn pin_matcher(x: char) -> Vec<String> {
        let found = match x {
            '0' => "08",
            '1' => "124",
            '2' => "1235",
            '3' => "236",
            '4' => "1457",
            '5' => "24568",
            '6' => "3569",
            '7' => "478",
            '8' => "57890",
            '9' => "689",
            _ => panic!(),
        };
        found.chars().map(String::from).collect()
    }
    let matches = observed
        .chars()
        .map(pin_matcher)
        .multi_cartesian_product()
        .map(|x| x.join(""))
        .collect();

    matches
}

fn meeting(s: &str) -> String {
    s.split(";")
        .map(|x| {
            x.to_uppercase()
                .split(":")
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .join(", ")
        })
        .sorted()
        .map(|u| format!("({u})"))
        .join("")
}

fn perimeter(n: u64) -> u64 {
    fn fib(term: u64, val: u64, prev: u64) -> u64 {
        if term == 0 {
            return prev;
        }
        fib(term - 1, val + prev, val)
    }
    4 * (fib(n + 3, 1, 0) - 1)
}

fn spinning_rings(inner_max: u64, outer_max: u64) -> u64 {
    if (inner_max < 1000) & (outer_max < 1000) {
        brute_force_spinnig(inner_max, outer_max)
    } else {
        println!("inner: {inner_max}, outer: {outer_max}");

        let base = min(&inner_max, &outer_max).to_owned();

        let bf = brute_force_spinnig(inner_max, outer_max);

        for i in ((base - 2050)..=(base - 2000)).step_by(2) {
            let result = brute_force_spinnig(inner_max - i, outer_max - i);
            print!("inner: {inner_max}, outer: {outer_max},     ");
            println!("i:    {i}, result: {result}");
        }

        bf
    }
}

fn brute_force_spinnig(inner_max: u64, outer_max: u64) -> u64 {
    let mut inner_ring = std::iter::once(0).chain((1..=inner_max).rev()).cycle();
    let mut outer_ring = (0..=outer_max).cycle();
    make_one_turn(&mut inner_ring, &mut outer_ring);
    let (mut inner_position, mut outer_position) = make_one_turn(&mut inner_ring, &mut outer_ring);
    let mut increments: u64 = 1;

    while inner_position != outer_position {
        (inner_position, outer_position) = make_one_turn(&mut inner_ring, &mut outer_ring);

        increments += 1;
    }
    increments
}

fn make_one_turn(
    inner_ring: &mut std::iter::Cycle<
        std::iter::Chain<std::iter::Once<u64>, std::iter::Rev<std::ops::RangeInclusive<u64>>>,
    >,
    outer_ring: &mut std::iter::Cycle<std::ops::RangeInclusive<u64>>,
) -> (u64, u64) {
    let inner_next = inner_ring.next();
    let outer_next = outer_ring.next();
    match (inner_next, outer_next) {
        (Some(u), Some(v)) => (u, v),
        _ => panic!(),
    }
}

fn solution(s: &str) -> String {
    s.chars()
        .map(|x| {
            if x.is_uppercase() {
                format!(" {x}")
            } else {
                x.to_string()
            }
        })
        .collect()
}

fn count_duplicates(text: &str) -> u32 {
    let max_couter = text.chars().collect::<Counter<_>>();
    (max_couter.values().max().unwrap() - 1) as u32
    // u32::try_from(max_couter - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    // #[test]
    // fn test_indivisibility() {
    //     assert_eq!(count_duplicates("indivisibility"), 1);
    // }

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }

    // #[test]
    // fn sample_spinning_rings() {
    //     assert_eq!(spinning_rings(2, 3), 5);
    //     assert_eq!(spinning_rings(5, 5), 3);
    //     assert_eq!(spinning_rings(2, 10), 13);
    //     assert_eq!(spinning_rings(10, 2), 10);
    //     assert_eq!(spinning_rings(7, 9), 4);
    //     assert_eq!(spinning_rings(20, 10), 16);
    //     assert_eq!(spinning_rings(40, 20), 31);
    //     assert_eq!(spinning_rings(80, 40), 61);

    //     assert_eq!(spinning_rings(200, 100), 151);
    //     assert_eq!(spinning_rings(300, 200), 251);
    //     assert_eq!(spinning_rings(400, 300), 351);
    //     assert_eq!(spinning_rings(500, 400), 451);
    //     assert_eq!(spinning_rings(600, 500), 551);
    //     assert_eq!(spinning_rings(700, 600), 651);

    //     assert_eq!(spinning_rings(201, 101), 101);
    //     assert_eq!(spinning_rings(301, 201), 151);
    //     assert_eq!(spinning_rings(401, 301), 201);
    //     assert_eq!(spinning_rings(501, 401), 251);
    //     assert_eq!(spinning_rings(601, 501), 301);
    //     assert_eq!(spinning_rings(701, 601), 351);

    //     assert_eq!(spinning_rings(400, 200), 301);
    //     assert_eq!(spinning_rings(800, 400), 601);

    //     assert_eq!(spinning_rings(20, 1), 41);
    //     assert_eq!(spinning_rings(20, 2), 21);
    //     assert_eq!(spinning_rings(20, 3), 39);
    //     assert_eq!(spinning_rings(20, 4), 18);
    //     assert_eq!(spinning_rings(20, 5), 39);
    //     assert_eq!(spinning_rings(20, 6), 21);
    //     assert_eq!(spinning_rings(20, 7), 37);
    //     assert_eq!(spinning_rings(20, 8), 15);
    //     assert_eq!(spinning_rings(20, 9), 36);
    //     assert_eq!(spinning_rings(20, 10), 16);
    //     assert_eq!(spinning_rings(20, 11), 33);
    //     assert_eq!(spinning_rings(20, 12), 17);
    //     assert_eq!(spinning_rings(20, 13), 35);
    //     assert_eq!(spinning_rings(20, 14), 18);
    //     assert_eq!(spinning_rings(20, 15), 29);
    //     assert_eq!(spinning_rings(20, 16), 19);
    //     assert_eq!(spinning_rings(20, 17), 30);
    //     assert_eq!(spinning_rings(20, 18), 20);
    //     assert_eq!(spinning_rings(20, 19), 31);
    //     assert_eq!(spinning_rings(20, 20), 21);

    //     assert_eq!(spinning_rings(2_u64.pow(24), 3_u64.pow(15)), 23951671 + 1);
    // }

    #[test]
    fn basics_perimeter() {
        assert_eq!(perimeter(5), 80);
        assert_eq!(perimeter(7), 216);
        assert_eq!(perimeter(20), 114624);
        assert_eq!(perimeter(30), 14098308);
    }

    #[test]
    fn basic_tests() {
        assert_eq!(meeting("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn"),
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
        assert_eq!(meeting("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell"),
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
        assert_eq!(meeting("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern"),
            "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
    }

    #[test]
    fn test_for_pins() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }

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
