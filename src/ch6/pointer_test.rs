fn main() {
    let val: i32 = 10;
    let val_ptr: *const i32 = &val;
    println!("val={}, *val={:?}", val, val_ptr);
}

