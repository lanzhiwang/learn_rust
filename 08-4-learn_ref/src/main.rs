//引用: 用法 &
//让我们创建一个指向值的应用，但是并不拥有它，因为不拥有这个值，所以，当引用离开其值指向的作用域后也不会被丢弃

fn calcute_length(s: &String) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(", world");  // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn main() {
    let s1 = String::from("hello");
    // let len = calcute_length(s1);  // expected `&String`, found struct `String`
    let len = calcute_length(&s1);
    println!("len = {}", len);  // len = 5
    println!("s1 = {}", s1);  // s1 = hello

    modify_s(&s1);  // types differ in mutability 类型的可变性不同

}