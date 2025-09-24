use std::env;
use uv_shims::run_uv;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    run_uv("pip", &args);
}
