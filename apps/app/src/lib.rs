#![no_std]

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod mynewt;

mod wrapper;

use wrapper::*;

extern crate panic_halt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    sys_init_start();

    sys_init_app();

    let mut blinky = wrapper::os_task_build();
    let blinky_stack = [0; 32];
    os_task_init(&mut blinky, b"blinky", blinky_fn, 10, &blinky_stack);

    sys_init_end();

    main_loop();
}

pub extern "C" fn blinky_fn(_arg: *mut ::cty::c_void) {
    gpio_init_out(LED_BLINK_PIN, 1);

    loop {
        /* Wait one second */
        os_delay_ms(1000);

        /* Toggle the LED */
        gpio_toggle(LED_BLINK_PIN);
    }
}
