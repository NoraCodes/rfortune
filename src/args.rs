use quotes::Quote;

#[derive(PartialEq)]
pub enum Mode {
    Execute,
    Initialize,
    List,
    Add
}

pub fn parse_args(args: Vec<String>) -> Result<(Mode, String, Option<Quote>), String>
{
    let mode: Mode;
    let database_path: String;
    let quote: Option<Quote>;

    if args.len() < 2 {
        return Err(format!("Usage: {0} init|exec|list [path to database] or {0} add [path to database] quote author [source]", args[0]));
    }

    if args.len() < 3 {
        database_path = ":memory:".into();
    } else {
        database_path = String::from(args[2].clone());
    };

    match args[1].as_str() {
        "init" | "initialize" => {mode = Mode::Initialize;}
        "exec" | "execute" => {mode = Mode::Execute;}
        "list" => {mode = Mode::List;}
        "add" => {mode = Mode::Add}
        &_ => {return Err("First argument must be init(ialize), exec(ute), or list.".into());}
    };

    if mode == Mode::Add {
        let source: Option<String>;
        if args.len() == 5 {
            source = None;
        } else if args.len() == 6 {
            source = Some(args[5].clone());
        } else {
            return Err("Not enough arguments for command add.".into());
        }
        quote = Some((args[3].clone(), args[4].clone(), source));
    } else {
        quote = None;
    }


    return Ok((mode, database_path, quote));
}
