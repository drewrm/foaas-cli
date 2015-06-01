pub fn get_path(command: String, from: Option<String>, name: Option<String>) -> String {
    let path = match &*command {
        "version" => format!("{command}", command=command),
        "operations" => format!("{command}", command=command),
        "this" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "that" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "pink" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "life" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "thanks" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "everything" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "everyone" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "flying" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "cool" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "bucket" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "diabetes" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "tucker" => format!("{command}/{from}", command=command, from=from.unwrap()),
        "awesome" => format!("{command}/{from}", command=command, from=from.unwrap()),
        _ => format!("{command}/{name}/{from}", command=command, from=from.unwrap(), name=name.unwrap())
    };

    return "http://www.foaas.com/".to_string() + &*path;
}
