fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); // hello
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i); // i = 5
}

fn main() {
    let s = String::from("hello");
    let s1 = takes_ownership(s);
    println!("{}", s1); // hello

    // println!("{}", s);  // value borrowed here after move

    let x = 5;
    makes_copy(x);
    println!("{}", x); // 5
    println!("Hello, world!");
}
