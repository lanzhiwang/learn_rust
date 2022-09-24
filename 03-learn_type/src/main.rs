fn main() {
    //bool
    let is_true: bool = true;
    println!("is_true = {:#?}", is_true); // is_true = true

    let is_false = false;
    println!("is_false = {:#?}, {:#?}", is_false, is_true); // is_false = false, true

    //char 在rust里面，char是32位的
    let a = 'a';
    println!("a = {:#?}", a); // a = 'a'

    let b = '你';
    println!("b = {:#?}", b); // b = '你'

    //i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
    let c: i8 = -111;
    println!("c = {:#?}", c); // c = -111

    let d: f32 = 0.0009;
    println!("d = {:#?}", d); // d = 0.0009

    //自适应类型isize, usize
    //数值类型: 有符号整数 (i8, i16, i32, i64, isize)、
    //无符号整数 (u8, u16, u32, u64, usize)
    //浮点数 (f32, f64)、以及有理数、复数
    println!("max = {:#?}", usize::max_value()); // max = 18446744073709551615

    //数组
    //[Type; size] , size 也是数组类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let arr1: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {:#?}", arr[0]); // arr[0] = 1
    println!("arr = {:?}", arr); // arr = [1, 2, 3, 4, 5]

    show(arr1);

    //元组
    let tup1: (i32, f32, char) = (-3, 3.69, '好');
    let tup = (-3, 3.69, '好');
    println!("tup1 = {:?}", tup1); // tup1 = (-3, 3.69, '好')
    println!("--------------------");
    println!("{:#?}", tup.0);
    println!("{:#?}", tup.1);
    println!("{:#?}", tup.2);
    println!("--------------------");
    // --------------------
    // -3
    // 3.69
    // '好'
    // --------------------

    let (x, y, z) = tup;
    println!("{:#?}", x); // -3
    println!("{:#?}", y); // 3.69
    println!("{:#?}", z); // '好'

    println!("Hello, world!");
}

fn show(arr: [u32; 3]) {
    println!("--------------------");
    for i in arr {
        println!("{:#?}", i);
    }
    println!("--------------------");
    for i in &arr {
        println!("{:#?}", i);
    }
    println!("--------------------");

    // --------------------
    // 1
    // 2
    // 3
    // --------------------
    // 1
    // 2
    // 3
    // --------------------
}
