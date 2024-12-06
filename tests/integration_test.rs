use calculator::operations::add;

#[test]
fn test_add() {
    let e: i32 = add(1, 2);
    assert_eq!(e, 3); // Example assertion
}
