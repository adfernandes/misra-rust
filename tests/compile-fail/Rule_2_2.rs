#[deny(dead_code)]

const DEAD_CODE: i32 = 0;
//~^ constant is never used: `DEAD_CODE` [dead_code]

fn main() {}
