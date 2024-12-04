mod problem1;
mod problem2;
mod problem3;

fn main() {
    problem1::p1::main(None);
    problem1::p2::main(None);
    problem2::p1::main(None);
    problem2::p2::main(None);
    problem3::p1::main(None);
    problem3::p2::main(Some(true));
}
