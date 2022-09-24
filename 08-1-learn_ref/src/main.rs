fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}", s1); // s1 = hello

    let mut s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // println!("s2 = {}", s2);  // value borrowed here after move
    println!("s3 = {}", s3); // s3 = hello

    s2 = takes_and_gives_back(s3);
    println!("s2 = {}", s2);  // s2 = hello

    println!("Hello, world!");
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}



