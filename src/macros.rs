macro_rules! call_jvmti {
    ($obj: expr, $func:ident $(,$($arg:expr),*)?) => {{
        let err_code = (**$obj).$func.expect(concat!(stringify!($func), " is not available"))($obj, $($($arg),*)?);
        $crate::prelude::native_call_result(err_code)
    }};
}

pub(crate) use call_jvmti;
