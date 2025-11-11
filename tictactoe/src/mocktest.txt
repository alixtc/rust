use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self) -> u32;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn working_mock_sequence() {
        let mut mock = MockMyTrait::new();
        let mut foo_values = [3, 3, 2].iter();
        mock.expect_foo()
            .returning(move || *foo_values.next().unwrap());

        assert_eq!(3, mock.foo());
        assert_eq!(3, mock.foo());
        assert_eq!(2, mock.foo());
    }
}
