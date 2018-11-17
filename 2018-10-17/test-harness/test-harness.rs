use std::ops::Add;

fn something<T>(a: T, b: T) -> T
    where T: Add<Output=T>
{
    a + b
}

#[test]
fn some_test() {
    assert_eq!(something(0, 0), 0);
}

#[test]
fn some_other_test() {
    assert_eq!(something(1, 2), 3);
}

fn main() {

}
