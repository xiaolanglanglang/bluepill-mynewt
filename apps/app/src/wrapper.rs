pub const LED_BLINK_PIN: i32 = 45;

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();
    fn hal_gpio_init_out(pin: i32, val: i32) -> i32;
    fn hal_gpio_toggle(pin: i32);
    fn os_time_delay(os_ticks: u32);
}

pub fn sys_init_start() {
    unsafe { sysinit_start() }
}

pub fn sys_init_end() {
    unsafe { sysinit_end() }
}

pub fn sys_init_app() {
    unsafe { sysinit_app() }
}

pub fn gpio_toggle(pin: i32) {
    unsafe { hal_gpio_toggle(pin) }
}

pub fn gpio_init_out(pin: i32, val: i32) -> i32 {
    unsafe { hal_gpio_init_out(pin, val) }
}

pub fn os_delay_ms(ms: u32) {
    unsafe { os_time_delay(ms) }
}
