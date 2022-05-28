/// Ergonomic wrapper around the js console.log using web_sys
#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! console_err {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}
