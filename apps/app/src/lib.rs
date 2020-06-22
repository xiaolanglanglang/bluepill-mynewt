#![no_std]

mod wrapper;

extern crate panic_halt;


#[no_mangle]
pub extern "C" fn main() -> ! {
    /* Initialize all packages. */
    wrapper::sys_init_start();
    wrapper::sys_init_app();
    wrapper::sys_init_end();

    wrapper::gpio_init_out(wrapper::LED_BLINK_PIN, 1);

    loop {
        /* Wait one second */
        wrapper::os_delay_ms(1000);

        /* Toggle the LED */
        wrapper::gpio_toggle(wrapper::LED_BLINK_PIN);
    }
}