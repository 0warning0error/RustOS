#![no_std]
#![no_main]


use core::arch::global_asm;



mod sbi;
global_asm!(include_str!("entry.asm"));


#[macro_use]
mod console;
mod lang_items;

#[no_mangle]
pub fn rust_main() -> ! {
	//这些符号定义在链接脚本里，意义是某段的初始地址和结尾地址
    extern "C"{
		fn stext();
		fn etext();
		fn srodata();
		fn erodata();
		fn sdata();
		fn edata();
		fn skernel();
		fn ekernel();
		fn sbss();
		fn ebss();
	}

	clear_bss();
	println!("\x1b[34m.text from {:#x} to {:#x}\x1b[0m",stext as usize,etext as usize,);
	println!(".rodata from {:#x} to {:#x}",srodata as usize,erodata as usize,);
	println!(".data from {:#x} to {:#x}",sdata as usize,edata as usize,);
	println!(".kernel from {:#x} to {:#x}",skernel as usize,ekernel as usize,);
	println!(".bss from {:#x} to {:#x}",sbss as usize,ebss as usize);
    println!("Hello, world!");
	panic!("Shutdown machine!");
}

fn clear_bss() {
	//初始化栈，全部置0
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}