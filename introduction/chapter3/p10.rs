// test

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
pub fn execute() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}
