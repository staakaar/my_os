#![no_std] // 標準ライブラリを無効
#![no_main] // すべてのRustレベルのエントリポイントを無効
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use my_os::{allocator, memory::{self, BootInfoFrameAllocator}, println, task::simple_executor::SimpleExecutor};
use x86_64::structures::paging::PageTable;
use my_os::task::{Task, simple_executor::SimpleExecutor};
use my_os::task::executor::Executor;
use my_os::task::keyboard;
extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use my_os::memory::active_level_4_table;
    use x86_64::{VirtAddr, structures::paging::Translate};
    use my_os::memory::translate_addr;

    println!("Hello World{}", "!");
    my_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // 未使用のページをまっぷする
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // 新しいマッピングを使って、文字列New!を画面に書き出す
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:?}", heap_value);

    // 動的サイズのベクタを作成
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:?}", vec.as_slice());

    // 参照カウントされたベクタを作成する-> カウントが0になると解放される
    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // let mut executor = SimpleExecutor::new();
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    my_os::hlt_loop();
}

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
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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

async fn async_number() -> u32 {
    42
}

async fn example_taslk() {
    let number = async_number().await;
    println!("async number: {}", number);
}