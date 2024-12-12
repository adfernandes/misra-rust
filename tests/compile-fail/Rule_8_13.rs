#[allow(unused_variables)]

fn main() {
    let mut x: Box<u8> = Box::new(8);
    //~^ ERROR variable does not need to be mutable [unused_mut]
}
