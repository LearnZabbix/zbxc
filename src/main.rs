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
use zabbix_api::host::create::{CreateHostRequest,CreateHostGroupResponse};
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
////	    // https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/create
////	    // create requests json using json.
////            //	        "jsonrpc": "2.0",
////            //    "method": "host.create",
////            //    "params": {
////            //        "host": "Linux server",
////            //        "interfaces": [
////            //            {
////            //                "type": 1,
////            //                "main": 1,
////            //                "useip": 1,
////            //                "ip": "192.168.3.1",
////            //                "dns": "",
////            //                "port": "10050"
////            //            }
////            //        ],
////            //        "groups": [
////            //            {
////            //                "groupid": "50"
////            //            }
////            //        ],
////            //        "tags": [
////            //            {
////            //                "tag": "Host name",
////            //                "value": "Linux server"
////            //            }
////            //        ],
////            //        "templates": [
////            //            {
////            //                "templateid": "20045"
////            //            }
////            //        ],
////            //        "macros": [
////            //            {
////            //                "macro": "{$USER_ID}",
////            //                "value": "123321"
////            //            },
////            //            {
////            //                "macro": "{$USER_LOCATION}",
////            //                "value": "0:0:0",
////            //                "description": "latitude, longitude and altitude coordinates"
////            //            }
////            //        ],
////            //        "inventory_mode": 0,
////            //        "inventory": {
////            //            "macaddress_a": "01234",
////            //            "macaddress_b": "56768"
////            //        }
////            //    },
////            //    "auth": "038e1d7b1735c6a5436ee9eae095879e",
////            //    "id": 1
////            //}
////            #[derive(Serialize)]
////            struct Host           {pub host: Vec<String>,}
////            struct Interfaces     {pub interface: Vec<String>,}
////            struct Groups         {pub groups: Vec<String>,}
////            struct Tags           {pub tags: Vec<String>,}
////            struct Templates      {pub templates: Vec<String>,}
////            struct Macros         {pub macros: Vec<String>,}
////	    struct InventoryMode  {pub inventory_mode: Vec<String>,}
////            struct Inventory      {pub inventory: Vec<String>,}
////            let request = CreateHostRequest {
////                host: "test01".to_owned(),
////                interface: Interfaces {vec!["Zabbix server".to_string()]},
////                groups: Vec<String>,
////                tags: Vec<String>,
////                templates: Vec<String>,
////                macros: Vec<String>,
////	        inventory_mode: 0 ,
////                inventory: Inventory {vec!["macaddress_a","macaddressb"]}
////            };
////            crate::functions::hello_world();
//////            let hostname = String::from(sub_matches.get_one::<String>("NAME").unwrap());
////            let http_client = ClientBuilder::new()
////                .danger_accept_invalid_certs(false)
////                .build()
////                .unwrap();
////            let client = ZabbixApiV6Client::new(http_client, DEFAULT_URL);
////            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD).unwrap();
////            match client.get_hosts(&session,&request) {
////                Ok(hosts) => {
////		    //  assert_eq!(hosts.len(), 1);
////                    println!("host.len()= {:?}",hosts.len());
////                    let host = hosts.first().unwrap();
////                    // assert_eq!(&host.host,"Zabbix server")
////                    println!("host: {:?}", host);
////                }
////                Err(e) => {
////                    eprintln!("host get error: {}", e);
////                    panic!("{}", e)
////                }
////            }
        } // create

        Some(("retrieve", sub_matches)) => {
            println!(
                "'retrieve' Command was used, Option is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
	    // https://www.zabbix.com/documentation/6.0/en/manual/api/reference/host/get
            //{ params in json format
            //    "jsonrpc": "2.0",
            //    "method": "host.get",
            //    "params": {
            //        "filter": {
            //            "host": [
            //                "Zabbix server",
            //                "Linux server"
            //            ]
            //        }
            //    },
            //    "auth": "038e1d7b1735c6a5436ee9eae095879e",
            //    "id": 1
            //}	    
            #[derive(Serialize)]
            struct Filter {
                pub host: Vec<String>,
            }
            let request = GetHostsRequest {
                filter: Filter {
                    host: vec!["Zabbix server".to_string()]
                },
            };
            crate::functions::hello_world();
//            let hostname = String::from(sub_matches.get_one::<String>("NAME").unwrap());
            let http_client = ClientBuilder::new()
                .danger_accept_invalid_certs(false)
                .build()
                .unwrap();
            let client = ZabbixApiV6Client::new(http_client, DEFAULT_URL);
            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD).unwrap();
            match client.get_hosts(&session,&request) {
                Ok(hosts) => {
		    //  assert_eq!(hosts.len(), 1);
                    println!("host.len()= {:?}",hosts.len());
                    let host = hosts.first().unwrap();
                    // assert_eq!(&host.host,"Zabbix server")
                    println!("host: {:?}", host);
                }
                Err(e) => {
                    eprintln!("host get error: {}", e);
                    panic!("{}", e)
                }
            }
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
            let session = client.get_auth_session(DEFAULT_ADMIN, DEFAULT_PASSWORD);	    
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
