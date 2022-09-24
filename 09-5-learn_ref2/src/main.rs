fn main() {
    let mut s = String::from("456");
    foo(&mut &mut &mut &mut &mut &mut s);
}

fn foo(s: &mut String) {
    let m = s;
    m.push_str("asd")
}
