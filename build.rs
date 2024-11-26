extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/libs/win_sys_api.c")
        .compile("win_sys_api");
}
