fn add()-> i32 {4}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add();
        assert_eq!(result, 4);
    }
}