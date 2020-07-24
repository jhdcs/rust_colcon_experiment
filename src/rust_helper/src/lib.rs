#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn gen_message(count: usize, mut s_out: &CxxString);
    }
}

fn gen_message(count: usize, mut s_out: &cxx::CxxString) {
    s_out = format!("Hello from rust! {}", count);
}
