enum Uniqueness {
    Red = 3,
    Blue,
    Green,
    Yellow = 5, //~ ERROR discriminant value `5` already exists
}

fn main() {}
