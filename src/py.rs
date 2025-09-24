use std::env;
use uv_shims::run_uv;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // Find out if there is -2 or -3 in the arguments, that is the version selector for python
    let version_position = args.iter().position(|x| x == "-2" || x == "-3");

    if let Some(pos) = version_position {
        // take out the version selector
        let mut args = args.clone();
        let version = args.remove(pos);

        if version == "-2" {
            run_uv("python2", &args);
        } else {
            run_uv("python", &args);
        }
    }
}
