#[cfg(test)]
mod tests;

use clap::{arg, command, Command};
mod classes;
mod functions;
mod hello;
use reqwest::blocking::ClientBuilder;
use zabbix_api::client::v6::ZabbixApiV6Client;
use zabbix_api::client::ZabbixApiClient;
//use zabbix_api::host::get::{GetHostGroupsRequest, GetHostsRequest};
use zabbix_api::host::get::GetHostsRequest;
use serde::Serialize;
const DEFAULT_URL: &str = "http://localhost:3080/api_jsonrpc.php";
const DEFAULT_ADMIN: &str = "Admin";
const DEFAULT_PASSWORD: &str = "zabbix";

fn main() {
    let matches = command!() 
        .help_template(
            "{before-help}{name}-{version} {about-with-newline}{author-with-newline} 
{usage-heading} [Options] [Commands] [Options] 

{all-args}{after-help} ",
        ) // requires clap `help` feature in Cargo.toml
        .version("1.1")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("C-RUD: to create a file.\n Ex: hello create test.txt")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("retrieve")
                .about("C-R-UD: to retrieve a file")
                .arg(arg!([NAME])),
        )
        .subcommand(Command::new("update").about("CR-U-D ...").arg(arg!([NAME])))
        .subcommand(
            Command::new("delete")
                .about("CRU-D : show zabbix session id")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("check")
                .about(
                    "check zabbix server status.\n Ex: check http://localhost:3080/api_jsonrpc.php \n Ex: check https://zabbix.test.com/zabbix/api_jsonrpc.php",
                )
                .arg(arg!([URL])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            println!(
                "'myapp create' Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            hello::hello(); // from hello.rs
            let circle = classes::Circle { radius: 3.0 };
            println!(
                "Circle: radius = {}, circumference = {}",
                circle.radius,
                circle.circumference()
            );
        } // create

        Some(("retrieve", sub_matches)) => {
            println!(
                "'retrieve' Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            crate::functions::hello_world();
//            let hostname = String::from(sub_matches.get_one::<String>("NAME").unwrap());
            let http_client = ClientBuilder::new()
                .danger_accept_invalid_certs(false)
                .build()
                .unwrap();
            let client = ZabbixApiV6Client::new(http_client, &DEFAULT_URL);
             #[derive(Serialize)]
            struct Filter {
                pub host: Vec<String>,
            }
            let request = GetHostsRequest {
                filter: Filter {
                    host: vec!["Zabbix server".to_string()]
                },
            };
            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD).unwrap();	    
            match client.get_hosts(&session,&request) {
                Ok(hosts) => {
                    assert_eq!(hosts.len(), 1);
                    let host = hosts.first().unwrap();
                    assert_eq!(&host.host,"Zabbix server")
                }
                Err(e) => {
                    eprintln!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
            crate::functions::hello_world();	    
        } // retrieve

        Some(("update", sub_matches)) => {
            println!(
                "'myapp update' Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            let result = functions::multiply(5, 3);
            println!("Product: {}", result);
        } // update

        Some(("delete", sub_matches)) => {
            println!(
                "'myapp delete' Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            let name = String::from("Rusty");
            crate::functions::greeting(name);
            let rectangle = classes::Rectangle {
                width: 10.0,
                height: 5.0,
            };
            println!(
                "Rectangle: width = {}, height = {}, area = {}",
                rectangle.width,
                rectangle.height,
                rectangle.area()
            );
        } //delete

        Some(("check", sub_matches)) => {
            let url = String::from(sub_matches.get_one::<String>("URL").unwrap());
            println!(
                "check Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("URL")
            );
//            let _ = functions::do_pings();
            println!("check Command was used, Option is: {:?}", &url);
            let http_client = ClientBuilder::new()
                .danger_accept_invalid_certs(false)
                .build()
                .unwrap();

            let client = ZabbixApiV6Client::new(http_client, &url);
	    //            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD).unwrap();
            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD);	    
	    //            match client.get_auth_session("Admin", "zabbix") {
            match session {
                Ok(session) => println!("session: {session}"),
                Err(e) => {
                    eprintln!("error: {}", e);
                    panic!("unexpected error")
                }
            }
            match client.get_api_info() {
                Ok(info) => println!("Zabbix API Server Version Info: {info}"),
                Err(e) => {
                    eprintln!("error: {}", e);
                    panic!("unexpected error")
                }
            }

        } // check

        _ => unreachable!("Exhausted list of Commands and Command_required prevents `None`"),
    }
}
