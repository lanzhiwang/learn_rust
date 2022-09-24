fn main() {
    let mut a = String::from("123");
    let b = &mut a;
    println!("a = {}, b = {}", a, b);
}

// error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
//  --> src/main.rs:4:32
//   |
// 3 |     let b = &mut a;
//   |             ------ mutable borrow occurs here
// 4 |     println!("a = {}, b = {}", a, b);
//   |                                ^  - mutable borrow later used here
//   |                                |
//   |                                immutable borrow occurs here
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

// Rust 所有权的管理相当于是同一时间只能有一个变量能够改值(拥有所有权) 可变引用一定程度上相当于获取了改权
