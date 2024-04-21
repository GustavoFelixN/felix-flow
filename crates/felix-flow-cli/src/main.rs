use std::io::{self, Write};

fn run(input: &str, env: &mut felix_flow::Env) -> Result<Option<felix_flow::Val>, String> {
    let parse = felix_flow::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated == felix_flow::Val::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    let mut env = felix_flow::Env::default();

    loop {
        write!(stdout, ">>> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Ok(None) => {}
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }

        input.clear();
    }
}
