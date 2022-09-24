fn main() {
    let mut s = String::from("456");
    let mut n = &mut s;
    foo(&mut n);
}

fn foo(s: &mut String) {
    let m = s;
    m.push_str("asd")
}
