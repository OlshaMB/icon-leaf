pub struct Formatter;

impl Formatter {
    fn format(icons: Vec<String>) -> String {
        icons.join(" ")
    }
    pub(crate) fn print(icons: Vec<String>) {
        print!("{}", Formatter::format(icons))
    }
}
