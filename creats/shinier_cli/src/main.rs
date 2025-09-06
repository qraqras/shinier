use bpaf::*;
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
            if paths.is_empty() {
                eprintln!("No paths provided for formatting.");
                std::process::exit(1);
            }
            for path in paths {
                if path.is_dir() {
                    // loop child files
                    for entry in std::fs::read_dir(&path).unwrap() {
                        println!("{:?}", entry.unwrap().file_name());
                    }
                }
                println!("{:?}", path.display());
            }
        }
    }
}
