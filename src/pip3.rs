use std::env;
use uv_shims::run_uv;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    args.insert(0, "-m".to_string());
    args.insert(1, "pip3".to_string());
    run_uv("python", &args);
}
