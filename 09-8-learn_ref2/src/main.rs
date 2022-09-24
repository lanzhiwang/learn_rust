fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // hello, world2, world1
}

fn change(some_string: &mut String) {
    (*some_string).push_str(", world2");
    some_string.push_str(", world1");
}