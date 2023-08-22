# RustCommandLineCalcuator
RustCommandLineCalcuator, or simply rclc for short is a fast, scriptable calculator that is 
designed to run right in your shell. No more need to use the python shell, or ugly and bloated
GUIs. Easily calculate complex formulas in your bash scripts.

Currently, rclc's status is `alpha`. This means that important major features are still missing
and bugs might not only be possible but common.

# Install
rclc is still in an early version, but if you wish, you can compile and install it using `cargo install --path .`, this will copy a release version to `$HOME/.cargo/bin`. Otherweise, you can compile rclc manually using `cargo build --release` and copy the binary in  `target/release/rclc` to a directory of your choice.