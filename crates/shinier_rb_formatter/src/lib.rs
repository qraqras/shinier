macro_rules! println_dbg {
    ($fmt:expr) => {
        if cfg!(debug_assertions) {
            (print!(concat!($fmt, "\n")));
        }
    };
}

mod builder;
mod doc;
mod formatter;
mod printer;
mod renderer;

pub use builder::*;
pub use doc::*;
pub use formatter::*;
pub use printer::*;
pub use renderer::*;
