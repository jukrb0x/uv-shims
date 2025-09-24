# uv-shims

The shims for [uv](https://docs.astral.sh/uv/), written in Rust.

Provide `python`, `python3`, `pip` and commands you are familiar with but running with `uv run <command>`

The shim for `pip` is not `uv pip` instead, it's `uv run python -m pip`. You can install pip in your project (or globally) via

```sh
uv add pip
```

This is for the compatibility for the users who still use `pip` for some projects.

Also we add the support for PyLauncher on windows (`py.exe` or `py`) that are able to select the python version with
syntax like `py -3`.
