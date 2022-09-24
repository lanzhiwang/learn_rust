fn main() {
    let mut s = String::from("456");
    let n = &mut s;
    foo(n);
}

fn foo(s: &mut String) {
    let m = s;
    m.push_str("asd")
}
