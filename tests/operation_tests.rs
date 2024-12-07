use calculator::operations::{add, subtract};

#[test]
fn test_add() {
    let e: i32 = add(1, 2);
    let f = add(1, 2);
    assert_eq!(e, f);
}
#[test]
fn test_substract() {
    let e: i32 = subtract(2, 1);
    assert_eq!(e, 1);
}
