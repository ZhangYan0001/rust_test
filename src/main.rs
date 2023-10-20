use advance_rs_learn::advance::{closure, lifetime};


fn main() {
    closure::test();
    lifetime::test();

    let i = 3;
    let mut _v = &3;
    let value1 = closure::add_one_closure(i);

    println!("the num add one is {}", value1);
}
