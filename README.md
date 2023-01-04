# range_checker

range_checker is a derive-macro crate aimed to provide declarative bounds checking and filtering for structure.

## Examples

```rust
use range_checker::Check;

#[derive(Debug, Default, Check)]
struct TestStruct {
    #[range(..=5)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    #[fallback(3.1)]
    v1: f32,
    #[filter(|x| x > 8.0)]
    #[fallback(9.9)]
    v2: f64,
    #[range(..-1)]
    #[fallback(|x| {println!("test fallback closure."); x - 5})]
    v3: isize,
}

fn main() {
    let a = TestStruct::default();

    if let Ok(_) = a.check() {
        todo!{}
    }
}
```

---

## Features

### **range**

Use `#[range(...)]` to detemine the range for named struct's field.

Multiple `#[range(...)]` will combined by `||`, which allowed to config any range you need.

### **filter**

Use `#[filter(#closure)]` to filter for named struct's field, `#closure: |#type_of_field| -> bool`.

Multiple `#[filter(#closure)]` will combined by `&&`.

`#[filter(#closure)]` will combined with `#[range(...)]` by `&&`.

### **fallback**
> Active with the `fn check_with_callback()`

Use  `#[fallback(#closure|#lit)]` to set fallback behavior for named struct's field, `#closure: |#type_of_field| -> #type_of_field`.

One field brokes it's range or filter with fallback attribute setted, `fn check_with_fallback()` will return `Ok(_)`.

---

## Check or CheckVerbose

`range_checker::Check` will return `Err(())` immediately any field broke it's range or filter.

`range_checker::CheckVerbose` will check all fields and return detail information like `Err(Vec<Error>)` or `Ok(Vec<Error>)` when all of broken fields has a fallback attribute.

---

## Expand Example

Here(https://github.com/TYPEmber/range_checker/tree/master/examples/expand_code.md) is an Expand Example for

```rust
use range_checker::Check;
use range_checker::CheckVerbose;

#[derive(Debug, Default, CheckVerbose)]
struct TestStruct {
    #[range(..=5)]
    #[range(20..)]
    #[filter(|x| x % 2 != 0)]
    #[fallback(255)]
    pub v0: u8,
    #[range(-1.0..=5.0)]
    #[fallback(3.1)]
    v1: f32,
    #[filter(|&x| x > 8.0)]
    #[fallback(9.9)]
    v2: f64,
    #[range(..-1)]
    #[fallback(|x| {println!("test fallback closure."); x - 5})]
    v3: isize,
}
```