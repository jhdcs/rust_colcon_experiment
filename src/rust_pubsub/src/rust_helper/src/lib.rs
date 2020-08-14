#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn gen_message(count: u32) -> String;
    }
}

fn gen_message(count: u32) -> String {
    format!("Hello from rust! {}\0", count)
}
