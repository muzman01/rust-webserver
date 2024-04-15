use clap::{Arg, Command};

extern crate cr8s;

#[tokio::main]
async fn main(){
    let matches = Command::new("Cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                        .about("User commands")
                        .arg_required_else_help(true)
                        .subcommand(
                            Command::new("create")
                                        .about("Create a new user")
                                        .arg_required_else_help(true)
                                        .arg(Arg::new("username").required(true))
                                        .arg(Arg::new("password").required(true))
                                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                        )
                        .subcommand(
                            Command::new("list")
                                        .about("List existing users")
                                        
                        )
                        .subcommand(
                            Command::new("delete")
                                        .about("Delete by user ID")
                                        .arg_required_else_help(true)
                                        .arg(Arg::new("id").required(true))
                                        
                                        
                        )
        )
        .get_matches();
    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand(){
            Some(("create", sub_matches)) => cr8s::command::create_users(
                sub_matches.get_one::<String>("username").unwrap().to_owned(),
                sub_matches.get_one::<String>("password").unwrap().to_owned(),
                sub_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect()).await,
            Some(("list", _)) => cr8s::command::list_users().await,
            Some(("delete", _)) => cr8s::command::delete_users(sub_matches.get_one::<i32>("id").unwrap().to_owned()).await,
            _ => {},
        },
        _ => {},
    }
}