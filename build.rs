extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/libs/win_sys_api.c")
        .compile("win_sys_api");
    // バグる時があるので、ビルドしても内容が変わらない場合は消す
    println!("cargo:rerun-if-changed=src/libs/win_sys_api.c");
}
