#[allow(unused_variables, unused_macros)]

macro_rules! val {
    () => {
        3;
    };
}

fn main() {
    let val!: i16 = 1; //~ ERROR expected one of `(`, `[`, or `{`, found `:`
}
