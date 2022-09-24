//1、在任意给定时间，有了可变引用之后不能再有不可变引用
//2、引用必须有效
fn main() {
    let mut x = 1;
    let y = &mut x;
    x = 2;
    *y = 3;
}

// error[E0506]: cannot assign to `x` because it is borrowed
//  --> src/main.rs:6:5
//   |
// 5 |     let y = &mut x;
//   |             ------ borrow of `x` occurs here
// 6 |     x = 2;
//   |     ^^^^^ assignment to borrowed `x` occurs here
// 7 |     *y = 3;
//   |     ------ borrow later used here

// 同一作用域，特定数据只能有一个可变引用
// 说这个可以避免竞争，但是直接修改原来的变量也可以改值，再加上那个可变引用，其实还是有两个地方可以修改变量的值，这样不会竞争么？