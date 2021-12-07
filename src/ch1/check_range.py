for s in ['i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64']:
    f = "    println!(\"N,{},{}\", N::min_value(), N::max_value());"
    f = f.replace("N", s)
    print(f)

    
