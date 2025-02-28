#[macro_export]
macro_rules! print_str {
    ($thread:expr, $dst:expr) => {
        {
            use $crate::runtime::lib::array_string::ArrayString;
            use core::fmt::Write;
            use $crate::runtime::protection_domain::thread::Thread;

            let mut message = ArrayString::<200>::default();
            let _ = message.write_str($dst);
            let char_array = unsafe{core::slice::from_raw_parts(message.as_ptr() as *const u8, message.len())};
            $thread.put_u8_array(&char_array);
        }
    };
    ($thread:expr, $dst:expr, $($args:tt)*) => {
        {
            use $crate::runtime::lib::array_string::ArrayString;
            use core::fmt::Write;
            use $crate::runtime::protection_domain::thread::Thread;

            let mut message = ArrayString::<200>::default();
            let _ = message.write_fmt(format_args!($dst, $($args)*));
            let char_array: &[u8] = unsafe{core::slice::from_raw_parts(message.as_ptr() as *const u8, message.len())};
            $thread.put_u8_array(char_array);
        }
    }
}
