use googletest::prelude::*;
struct MyFixture {
    common_data: i32,
}

impl ConsumableFixture for MyFixture {
    fn set_up() -> googletest::Result<Self> {
        todo!()
    }
}

#[gtest]
fn test1(fixture: MyFixture) {
    println!("Running test1...");
    assert_eq!(fixture.common_data, 42);
}

#[gtest]
fn test2(fixture: MyFixture) {
    println!("Running test2...");
    assert_eq!(fixture.common_data + 1, 43);
}
