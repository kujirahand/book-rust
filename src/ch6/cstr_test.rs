use std::ffi::CString;
fn main() {
    // Rustの文字列を用意
    let msg = "こんにちは。";
    // C言語の文字列に変換
    let msg_cstr = CString::new(msg).unwrap();
    // Cのライブラリを呼び出す
    unsafe {
        // ここでCのライブラリを呼び出す
        // c_lang_lib::print_str(msg_cstr.as_ptr());
    }
}

