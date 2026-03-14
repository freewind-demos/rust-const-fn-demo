# Rust Const Fn Demo

## 简介

演示 Rust const fn 编译时常量函数。

## 基本原理

const fn 在编译时求值，用于数组大小、静态变量等。

## 启动和使用

```bash
cargo run
```

## 教程

### const 函数

```rust
const fn const_add(a: i32, b: i32) -> i32 {
    a + b
}
const VALUE: i32 = const_add(10, 20);
```

### 作为数组大小

```rust
const SIZE: usize = 10;
let arr: [i32; SIZE] = [0; SIZE];
```

### const 方法

```rust
impl MyNum {
    const fn new(value: i32) -> Self {
        MyNum(value)
    }
}
```

### 限制

- 不能有副作用
- 只能做纯计算
- 不能调用非 const fn
