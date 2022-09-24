//引用: 用法 &
//让我们创建一个指向值的应用，但是并不拥有它，因为不拥有这个值，所以，当引用离开其值指向的作用域后也不会被丢弃

//借用: &mut

fn main() {
    let mut s1 = String::from("hello");
    let r1 = &s1; // 引用, 不可变引用
    let r2 = &s1; // 引用, 不可变引用
    println!("{}, {}", r1, r2); // hello, hello

    let r3 = &mut s1; // 借用, 可变引用
    r3.push_str(", world");
    println!("{}", r3); // hello, world
}
