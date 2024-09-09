macro_rules! jvmti {
    ($obj: expr, $func:ident $(,$($arg:expr),*)?) => {{
        let err_code = unsafe { (**$obj).$func.expect(concat!(stringify!($func), " is not available"))($obj, $($($arg),*)?) };
        err_code
    }};
}

pub(crate) use jvmti;
