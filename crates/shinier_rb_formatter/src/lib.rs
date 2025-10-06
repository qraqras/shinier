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
mod utility;

pub use builder::*;
pub use doc::*;
pub use formatter::*;
pub use renderer::*;
pub use utility::*;
