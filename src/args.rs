pub enum Mode {
    Execute,
    Initialize,
    List,
    Add
}

pub fn parse_args(args: Vec<String>) -> Result<(Mode, String), String>
{
    let mode: Mode;
    let database_path: String;

    if args.len() < 2 {
        return Err(format!("Usage: {0} init|exec|list [path to database] or {0} add [path to database]", args[0]));
    }

    match args[1].as_str() {
        "init" | "initialize" => {mode = Mode::Initialize;}
        "exec" | "execute" => {mode = Mode::Initialize;}
        "list" => {mode = Mode::List;}
        &_ => {return Err("First argument must be init(ialize), exec(ute), or list.".into());}
    }

    if args.len() < 3 {
        database_path = ":memory:".into();
    } else {
        database_path = String::from(args[2].clone());
    };

    return Ok((mode, database_path));
}
