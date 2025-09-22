use bpaf::*;
use shinier_rb_formatter::Formatter;
use std::path::PathBuf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum ShinierCommand {
    #[bpaf(command("format"), short('f'), long("format"))]
    Format {
        #[bpaf(positional("PATH"))]
        path: PathBuf,
    },
}

fn main() {
    let command = shinier_command().fallback_to_usage().run();
    match command {
        ShinierCommand::Format { path } => {
            Formatter::new(path, ()).format();
        }
    }
}
