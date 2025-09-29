#[macro_export]
macro_rules! errorln {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        println!("[{}]: {}", color_eyre::owo_colors::OwoColorize::red(&"Error"), format!($($arg)*))
    }};
}

#[macro_export]
macro_rules! infoln {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        println!("[{}]: {}", color_eyre::owo_colors::OwoColorize::cyan(&"Info"), format!($($arg)*))
    }};
}
