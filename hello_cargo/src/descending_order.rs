use itertools::Itertools;

pub fn descending_order(x: u64) -> u64 {
    x.to_string()
        .chars()
        .sorted()
        .rev()
        .join("")
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
    }

    #[test]
    fn it_works_with_larger_sequence() {
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
    }

    #[test]
    fn it_works_on_largest_sequences() {
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
