use std::env;
use std::process::{exit, Command};

pub fn run_uv(forward_to: &str, args: &Vec<String>) {
    // Remove "--user" when invoking pip since we use uv venv
    let mut filtered_args: Vec<String> = args.clone();

    // Determine if the invocation effectively targets pip:
    // - direct: forward_to == "pip" or "pip3"
    // - via python: forward_to starts with "python" and args contain "-m" followed by "pip" or "pip3"
    let mut is_effective_pip = forward_to == "pip" || forward_to == "pip3";
    if !is_effective_pip && forward_to.starts_with("python") {
        let mut i = 0;
        while i < filtered_args.len() {
            if filtered_args[i] == "-m" && i + 1 < filtered_args.len() {
                let mod_name = filtered_args[i + 1].as_str();
                if mod_name == "pip" || mod_name == "pip3" {
                    is_effective_pip = true;
                    break;
                }
                i += 2;
                continue;
            }
            i += 1;
        }
    }

    if is_effective_pip {
        filtered_args.retain(|a| a != "--user");
    }

    // Print the command with uv run
    println!(
        "[uv-shims] uv run {} {}",
        forward_to,
        filtered_args.join(" ")
    );

    let status = Command::new("uv")
        .arg("run")
        .arg(forward_to)
        .args(&filtered_args)
        .envs(env::vars())
        .status()
        .expect("Failed to execute 'uv run'");

    exit(status.code().unwrap_or(1));
}
