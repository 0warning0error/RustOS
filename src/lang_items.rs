
use crate::println;
use crate::sbi::shutdown;
use core::panic::PanicInfo;
use core::fmt::Debug;

//处理panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("panic occurred: {:?}",s);
    } else if let Some(s) = info.payload().downcast_ref::<&(dyn Debug)>()  {
        println!("panic occurred: {:?}", s);
    }else{
		println!("panic occurred, {:?}",info);
	}
    shutdown()
}

