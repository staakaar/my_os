#![no_std] // 標準ライブラリを無効
#![no_main] // すべてのRustレベルのエントリポイントを無効

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World";

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    // リンカはデフォルトで_startという名前を返すため
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    vga_buffer::print_something();
    loop {}
}

// パニック時にコール
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}