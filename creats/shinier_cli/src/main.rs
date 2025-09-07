use bpaf::*;
use shinier_formatter::execute_formatting;
use std::path::PathBuf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum ShinierCommand {
    #[bpaf(command("format"), short('f'), long("format"))]
    Format {
        #[bpaf(positional("PATHS"))]
        paths: Vec<PathBuf>,
    },
}

fn main() {
    let command = shinier_command().fallback_to_usage().run();
    match command {
        ShinierCommand::Format { paths } => {
            execute_formatting(paths);
        }
    }
}
