pub fn parse_args(args: Vec<String>) -> Result<(bool, String), String>
{
    let do_init: bool;
    let database_path: String;

    if args.len() < 2 {
        return Err(format!("Usage: {0} init [path to database] or {0} exec [path to database]", args[0]));
    }

    match args[1].as_str() {
        "init" | "initialize" => {do_init = true;}
        "exec" | "execute" => {do_init = false;}
        &_ => {return Err("First argument must be either init(ialize) or exec(ute).".into());}
    }

    if args.len() < 3 {
        database_path = ":memory:".into();
    } else {
        database_path = String::from(args[2].clone());
    };

    return Ok((do_init, database_path));
}
