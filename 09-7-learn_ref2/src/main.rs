fn main() {
    let mut a = String::from("123");

    // let b = &a;
    // println!("a = {},b = {}", a, b);  // a = 123,b = 123

    {
        let b = &a;
        println!("a = {},b = {}", a, b);  // a = 123,b = 123
    }

    {
        let b = &mut a;
        // println!("a = {}, b = {}", a, b); // 同一个值的两个可变的变量同时使用
        // error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
        //   --> src/main.rs:14:36
        //    |
        // 13 |         let b = &mut a;
        //    |                 ------ mutable borrow occurs here
        // 14 |         println!("a = {}, b = {}", a, b); // 同一个值的两个可变的变量同时使用
        //    |                                    ^  - mutable borrow later used here
        //    |                                    |
        //    |                                    immutable borrow occurs here
        //    |
        //    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

        println!("b = {}", b); // 这两行不能乱序 b = 123
        println!("a = {}", a); // 此时 b 已经结束了生命周期 (NLL) a = 123
    }
}