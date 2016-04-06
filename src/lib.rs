#[cfg(not(test))] extern crate impl_prod as implementation;
#[cfg(test)] extern crate impl_test as implementation;

use implementation::Impl;

pub fn do_foo() -> String {
    let foo = Impl;
    foo.foo()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_do_foo_with_test_impl() {
        assert_eq!("foo TEST", do_foo());
    }
}
