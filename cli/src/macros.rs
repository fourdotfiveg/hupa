
#[macro_export]
macro_rules! writef {
    ( $ dst : expr , $ ( $ arg : tt ) * ) => {
        {
            write!($dst, $($arg)*).unwrap();
            $dst.flush().unwrap();
        }
    };
}
