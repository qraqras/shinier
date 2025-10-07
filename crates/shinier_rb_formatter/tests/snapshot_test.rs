use insta::{assert_snapshot, glob};
use shinier_rb_formatter::Printer;

#[test]
fn run() {
    glob!("**/*.rb", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        let output = Printer::new(contents, ()).print();
        assert_snapshot!(output);
    });
}
