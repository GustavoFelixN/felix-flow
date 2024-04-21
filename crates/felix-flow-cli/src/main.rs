use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();

    loop {
        write!(stdout, ">>> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match felix_flow::parse(&input.trim()) {
            Ok(parse) => {
                dbg!(parse);
            }
            Err(msg) => {
                writeln!(stderr, "Parse error: {}", msg)?;
            }
        }

        input.clear();
    }
}
