use super_duper_octo_lamp::libs::calculator::operations as op;

#[test]
fn test_add() {
    let e: i32 = op::add(1, 2);
    let f = op::add(1, 2);
    assert_eq!(e, f);
}
#[test]
fn test_substract() {
    let e: i32 = op::subtract(2, 1);
    assert_eq!(e, 1);
}
