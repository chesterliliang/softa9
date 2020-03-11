extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/driver/os_uart.c")
        .file("src/driver/uart.c")
        .compile("libcom.a");
}
