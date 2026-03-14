fn main() {
    println!("=== Rust const fn 编译时常量演示 ===\n");

    // 1. const 函数 - 在编译时求值
    println!("--- const 函数 ---");
    const VALUE: i32 = const_add(10, 20);
    println!("const_add(10, 20) = {}", VALUE);

    // 2. 作为数组大小
    println!("\n--- 作为数组大小 ---");
    const SIZE: usize = 10;
    let arr: [i32; SIZE] = [0; SIZE];
    println!("数组长度: {}", arr.len());

    // 3. 在常量上下文中使用
    println!("\n--- 常量上下文 ---");
    const DOUBLES: [i32; 5] = double_array(&[1, 2, 3, 4, 5]);
    println!("双倍数组: {:?}", DOUBLES);

    // 4. 静态变量
    println!("\n--- 静态变量 ---");
    static COUNTER: u32 = const_counter();
    println!("静态计数器初始值: {}", COUNTER);

    // 5. const 泛型（Rust 1.51+）
    println!("\n--- const 泛型 ---");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("前3个元素: {:?}", get_first_n(&arr, 3));

    // 6. const 闭包（不支援，但可以模拟）
    // 7. const 内部可变性
    use std::cell::UnsafeCell;
    const CELL: UnsafeCell<i32> = UnsafeCell::new(42);
    println!("UnsafeCell: {}", unsafe { *CELL.get() });

    // 8. 与 let 的区别
    println!("\n--- const vs let ---");
    let runtime_value = add(1, 2); // 运行时计算
    const compile_value: i32 = add(1, 2); // 编译时计算
    println!("runtime: {}, compile: {}", runtime_value, compile_value);

    // 9. const 方法
    println!("\n--- const 方法 ---");
    let num = MyNum(42);
    println!("const method: {}", num.get_value());

    println!("\n=== 总结 ===");
    println!("const fn 在编译时求值");
    println!("用于数组大小、静态变量等");
    println!("const 方法可以在常量上下文中调用");
    println!("限制：不能有副作用，只能做纯计算");
}

// const 函数
const fn const_add(a: i32, b: i32) -> i32 {
    a + b
}

// 编译时计算数组
const fn double_array(arr: &[i32]) -> [i32; 5] {
    [arr[0] * 2, arr[1] * 2, arr[2] * 2, arr[3] * 2, arr[4] * 2]
}

// const 静态变量
const fn const_counter() -> u32 {
    0
}

// const 泛型（模拟）
fn get_first_n<const N: usize>(arr: &[i32], _n: usize) -> [i32; N] {
    let mut result = [0; N];
    let mut i = 0;
    while i < N && i < arr.len() {
        result[i] = arr[i];
        i += 1;
    }
    result
}

// 运行时函数（对比）
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// const 方法
struct MyNum(i32);

impl MyNum {
    const fn new(value: i32) -> Self {
        MyNum(value)
    }

    const fn get_value(&self) -> i32 {
        self.0
    }
}
