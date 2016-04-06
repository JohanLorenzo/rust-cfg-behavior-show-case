extern crate cfg_bug;

#[cfg(test)]
mod tests {
    use cfg_bug::do_foo;

    #[test]
    fn it_should_do_foo_with_test_impl() {
        assert_eq!("foo TEST", do_foo());
    }
}
