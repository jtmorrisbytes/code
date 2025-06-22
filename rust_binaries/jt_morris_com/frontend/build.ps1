param($out_dir = "$PsScriptRoot/dest")
$rust_target = "wasm32-unknown-unknown"
$crate_name = "login_page"
rustup update
rustup target add $rust_target
cargo install wasm-pack
cargo build -p $crate_name --release --target wasm32-unknown-unknown
$wasm_file = "$(Get-Location)\target\$rust_target\release\$crate_name.wasm"
wasm-bindgen  --target web --out-dir $out_dir $wasm_file