#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

enum Doc {
    String(String),
    Line,
    Indent(Vec<Doc>),
    Group(Vec<Doc>),
}

impl Doc {
    pub fn as_cmd(&self, ind: i32, mode: Mode) -> Command {
        Command {
            ind,
            doc: self,
            mode,
        }
    }
}

struct Command<'a> {
    pub ind: i32,
    pub doc: &'a Doc,
    pub mode: Mode,
}

fn render(doc: &Doc) -> String {
    let mut result = String::new();
    let mut cmds = Vec::new();
    cmds.push(doc.as_cmd(0, Mode::Break));

    while let Some(cmd) = cmds.pop() {
        match cmd.doc {
            Doc::String(text) => {
                result.push_str(text);
            }
            Doc::Line => match cmd.mode {
                Mode::Flat => result.push(' '),
                Mode::Break => {
                    result.push('\n');
                    result.push_str(&" ".repeat(cmd.ind as usize));
                }
            },
            Doc::Indent(contents) => {
                for d in contents.iter().rev() {
                    cmds.push(d.as_cmd(cmd.ind + 2, cmd.mode));
                }
            }
            Doc::Group(contents) => {
                // グループが1行に収まるか判定する必要がある
                // ここでは簡略化のため Break モードで処理
                for d in contents.iter().rev() {
                    cmds.push(d.as_cmd(cmd.ind, Mode::Break));
                }
            }
        }
    }

    result
}
