use crate::mynewt::os;
use crate::mynewt::os::{os_task, os_stack_t};
use cty::c_char;
use core::ffi::c_void;

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

pub fn os_task_build() -> os::os_task {
    unsafe {
        core::mem::transmute::<[u8; core::mem::size_of::<os::os_task>()], os_task>(
            [0; core::mem::size_of::<os::os_task>()]
        )
    }
}

pub fn os_task_init(
    task: &mut os::os_task,
    task_name: &[u8],
    task_fn: extern "C" fn(*mut c_void),
    priority: u8,
    task_stack: &[os::os_stack_t],
) {
    unsafe {
        os::os_task_init(
            task,                                   //  Task object will be saved here
            task_name.as_ptr() as *const c_char,    //  Name of task
            Some(task_fn),                          //  Function to execute when task starts
            0 as *mut c_void,                       //  Argument to be passed to above function
            priority,                               //  Task priority: highest is 0, lowest is 255 (main task is 127)
            os::OS_WAIT_FOREVER as u32,             //  Don't do sanity / watchdog checking
            task_stack.as_ptr() as *mut os_stack_t, //  Stack space for the task
            task_stack.len() as u16,                //  Size of the stack (in 4-byte units)
        );
    }
}

pub fn main_loop() -> ! {
    loop {
        unsafe {
            os::os_eventq_run(os::os_eventq_dflt_get())
        }
    }
}