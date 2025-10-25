use clap::{Arg, Command};

fn main() {
    let app = build_app();
    let matches = app.get_matches();
}

fn build_app() -> Command {
    Command::new("Clap Example")
        .author("user@example.com")
        .version("0.1.0")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("name")
                .long("name")
                .help("Sets the name to use")
                .required(false)
        )
        .after_help("Longer explanation to appear after the options when displaying the help information from --help or -h")
}
