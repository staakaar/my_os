#![no_std] // 標準ライブラリを無効
#![no_main] // すべてのRustレベルのエントリポイントを無効

use core::panic::PanicInfo;


#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで_startという名前を返すため
    loop {
        
    }
}

// パニック時にコール
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}