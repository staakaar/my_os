#![no_std] // 標準ライブラリを無効
#![no_main] // すべてのRustレベルのエントリポイントを無効
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_os::println;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

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

    println!("Hello World {}", "!");

    // 割り込み例外ハンドラーの初期化
    my_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address);

    let ptr = 0xdeadbeaf as *mut u8;
    unsafe { *ptr = 42; }

    // ブレイクポイント
    x86_64::instructions::interrupts::int3();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    my_os::hlt_loop();
}

// パニック時にコール
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    my_os::hlt_loop();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion...");
    asseert_eq!(1, 1);
    serial_println!("[ok]");
}
