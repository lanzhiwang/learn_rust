//1、字符串slice是String中一部分值的引用
//2、字面值就是slice
//3、其它类型slice
fn main() {
    // 字符串 slice 是 String 中一部分值的引用
    let s = String::from("hello world");

    let h = &s[0..5];
    println!("h = {}", h);
    let h = &s[0..=4];
    println!("h = {}", h);
    let h = &s[..=4];
    println!("h = {}", h);
    let h = &s[..5];
    println!("h = {}", h);
    println!("=========================");
    // h = hello
    // h = hello
    // h = hello
    // h = hello
    // =========================

    let w = &s[6..11];
    println!("w = {}", w);
    let w = &s[6..=10];
    println!("w = {}", w);
    let w = &s[6..];
    println!("w = {}", w);
    let w = &s[..];
    println!("w = {}", w);
    println!("=========================");
    // w = world
    // w = world
    // w = world
    // w = hello world
    // =========================

    // let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let mut c = &s[0..1]; // c 的类型是 slice, 属于可变引用
    println!("c = {}", c);
    println!("s = {}", s);
    c = &s[2..3];
    println!("c = {}", c);
    println!("s = {}", s);
    // c = &arr[2..5];  // expected `str`, found slice `[u32]`
    // println!("c = {}", c);
    // println!("s = {}", s);
    c = "w";
    println!("c = {}", c);
    println!("s = {}", s);
    println!("=========================");
    // c = h
    // s = hello world
    // c = l
    // s = hello world
    // c = w
    // s = hello world
    // =========================

    let ss = String::from("你好");
    // let w1 = &ss[0..1];
    let w1 = &ss[0..3];
    println!("w1 = {}", w1);
    println!("=========================");
    // w1 = 你
    // =========================

    // 字面值就是 slice, 也就是 &str
    let mut s3 = "hh"; //&str
    println!("s3 = {}", s3);
    s3 = "ll";
    println!("s3 = {}", s3);
    s3 = &s[6..11];
    println!("s3 = {}", s3);
    println!("=========================");
    // s3 = hh
    // s3 = ll
    // s3 = world
    // =========================

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {}", sss[0]);
    println!("sss = {}", sss[1]);
    println!("len = {}", sss.len());
    // sss = 2
    // sss = 3
    // len = 2

    println!("Hello, world!");
}
