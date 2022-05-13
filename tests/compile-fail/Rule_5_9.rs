const count: i32 = 0;

fn main() {
    let count: i32 = 1;
    //~^ ERROR refutable pattern in local binding: `i32::MIN..=-1_i32` and `1_i32..=i32::MAX` not covered [E0005]
}
