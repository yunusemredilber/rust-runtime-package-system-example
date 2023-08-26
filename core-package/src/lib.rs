use libc::size_t;

#[repr(C)]
pub enum FFIValue {
    Number(f64),
    Boolean(bool),
    Character(char),
    Void,
}

#[no_mangle]
pub extern "C" fn add_numbers(args: *mut FFIValue, len: size_t) -> FFIValue {
    let value1 = unsafe { &*args.offset(0) };
    let value2 = unsafe { &*args.offset(1) };

    if let (FFIValue::Number(_value1), FFIValue::Number(_value2)) = (value1, value2) {
        return FFIValue::Number(_value1 + _value2);
    }

    FFIValue::Number(0.0)
}

#[no_mangle]
pub extern "C" fn print(args: *mut FFIValue, len: size_t) -> FFIValue {
    let mut str = String::new();

    for n in (std::ops::Range { start: 0, end: len }) {
        let char = unsafe { &*args.offset(n as isize) };
        if let FFIValue::Character(_char) = char {
            str.push(*_char);
        }
    }

    println!("{}", str);

    FFIValue::Void
}
