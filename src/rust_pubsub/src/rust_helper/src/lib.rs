#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn gen_message(count: usize) -> String;
    }
}

fn gen_message(count: usize) -> String {
    format!("Hello from rust! {}", count)
}
