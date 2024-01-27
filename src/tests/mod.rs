// use log::{debug, error, info};
use log::{error, info};
use reqwest::blocking::Client;
// use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
// use zabbix_api::client::jsonrpc::{ZabbixApiRequest, ZabbixApiResponse};
// use zabbix_api::client::post::send_post_request;
use zabbix_api::client::ZabbixApiClient;
// use zabbix_api::error::ZabbixApiError;
//use zabbix_api::host::create::{
//    CreateHostGroupRequest, CreateHostGroupResponse, CreateHostRequest, CreateHostResponse,
//};

use zabbix_api::host::create::{CreateHostGroupRequest, CreateHostRequest};
use zabbix_api::host::{ZabbixHost, ZabbixHostGroup};
//use zabbix_api::item::create::{CreateItemRequest, CreateItemResponse};
use zabbix_api::item::create::CreateItemRequest;
// use zabbix_api::item::ZabbixItem;
// use zabbix_api::trigger::create::{CreateTriggerRequest, CreateTriggerResponse};
use zabbix_api::trigger::create::CreateTriggerRequest;
// use zabbix_api::trigger::ZabbixTrigger;
// use zabbix_api::webscenario::create::{CreateWebScenarioRequest, CreateWebScenarioResponse};
use zabbix_api::webscenario::create::CreateWebScenarioRequest;
// use zabbix_api::webscenario::ZabbixWebScenario;

// const JSON_RPC_VERSION: &str = "2.0";
use std::error::Error;
//use log::{error, info};
use zabbix_api::client::v6::ZabbixApiV6Client;
// use zabbix_api::client::ZabbixApiClient;
use zabbix_api::host::get::{GetHostGroupsRequest, GetHostsRequest};
//use zabbix_api::host::ZabbixHost;
//use zabbix_api::item::create::CreateItemRequest;
use zabbix_api::item::get::GetItemsRequest;
//use zabbix_api::tests::builder::TestEnvBuilder;
// use zabbix_api::test::integration::{are_integration_tests_enabled, get_integration_tests_config};
//use zabbix_api::tests::{get_random_string, init_logging};
//use zabbix_api::trigger::create::CreateTriggerRequest;
use zabbix_api::trigger::get::GetTriggerByIdRequest;
//use zabbix_api::webscenario::create::CreateWebScenarioRequest;
use zabbix_api::webscenario::get::GetWebScenarioByIdRequest;
use zabbix_api::webscenario::ZabbixWebScenarioStep;
use zabbix_api::ZABBIX_EXTEND_PROPERTY_VALUE;

// integration

use std::env;

const ENV_ZABBIX_API_URL: &str = "ZABBIX_API_URL";
const ENV_ZABBIX_API_USER: &str = "ZABBIX_API_USER";
const ENV_ZABBIX_API_PASSWORD: &str = "ZABBIX_API_PASSWORD";

pub fn are_integration_tests_enabled() -> bool {
    let result = env::var(ENV_ZABBIX_API_URL).is_ok()
        && env::var(ENV_ZABBIX_API_USER).is_ok()
        && env::var(ENV_ZABBIX_API_PASSWORD).is_ok();

    if !result {
        println!("/!\\ integration tests are disabled /!\\")
    }

    result
}

pub struct IntegrationTestsConfig {
    pub zabbix_api_url: String,
    pub zabbix_api_user: String,
    pub zabbix_api_password: String,
}

pub fn get_integration_tests_config() -> IntegrationTestsConfig {
    IntegrationTestsConfig {
        zabbix_api_url: env::var(ENV_ZABBIX_API_URL).unwrap(),
        zabbix_api_user: env::var(ENV_ZABBIX_API_USER).unwrap(),
        zabbix_api_password: env::var(ENV_ZABBIX_API_PASSWORD).unwrap(),
    }
}

// original mod.rs
use fake::{Fake, Faker};
use log::LevelFilter;

pub fn init_logging() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init();
}

pub fn get_random_string() -> String {
    Faker.fake::<String>()
}

//mod builder;
//use crate::client::v6::ZabbixApiV6Client;
//use crate::client::ZabbixApiClient;
//use crate::host::create::{CreateHostGroupRequest, CreateHostRequest};
//use crate::host::ZabbixHostGroup;
//use crate::item::create::CreateItemRequest;
//use crate::tests::integration::{get_integration_tests_config, IntegrationTestsConfig};
//use crate::trigger::create::CreateTriggerRequest;
//use crate::webscenario::create::CreateWebScenarioRequest;
//use crate::webscenario::ZabbixWebScenarioStep;

pub struct TestEnvBuilder {
    pub client: ZabbixApiV6Client,
    pub integration_tests_config: IntegrationTestsConfig,
    pub session: String,

    pub latest_host_group_id: u32,
    pub latest_host_id: u32,
    pub latest_item_id: u32,
    pub latest_trigger_id: u32,
    pub latest_webscenario_id: u32,
}

impl TestEnvBuilder {
    pub fn build() -> TestEnvBuilder {
        init_logging();

        let http_client = Client::new();

        let tests_config = get_integration_tests_config();

        TestEnvBuilder {
            client: ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url),
            integration_tests_config: tests_config,
            session: "".to_string(),
            latest_host_group_id: 0,
            latest_host_id: 0,
            latest_item_id: 0,
            latest_trigger_id: 0,
            latest_webscenario_id: 0,
        }
    }

    pub fn get_session(&mut self) -> &mut Self {
        match self.client.get_auth_session(
            &self.integration_tests_config.zabbix_api_user,
            &self.integration_tests_config.zabbix_api_password,
        ) {
            Ok(session) => {
                self.session = session;
                self
            }
            Err(e) => {
                error!("auth error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_host_group(&mut self, name: &str) -> &mut Self {
        let request = CreateHostGroupRequest {
            name: name.to_string(),
        };

        match &self.client.create_host_group(&self.session, &request) {
            Ok(host_group_id) => {
                self.latest_host_group_id = host_group_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("host group create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_host(&mut self, name: &str) -> &mut Self {
        let params = CreateHostRequest {
            host: name.to_string(),
            groups: vec![ZabbixHostGroup {
                name: "".to_string(),
                group_id: self.latest_host_group_id.to_string(),
            }],
            interfaces: vec![],
            tags: vec![],
            templates: vec![],
            macros: vec![],
            inventory_mode: 0,
            inventory: HashMap::new(),
        };

        match &self.client.create_host(&self.session, &params) {
            Ok(host_id) => {
                self.latest_host_id = host_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("host create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_item(&mut self, name: &str, key_: &str) -> &mut Self {
        let params = CreateItemRequest {
            name: name.to_string(),
            key_: key_.to_string(),
            host_id: self.latest_host_id.to_string(),
            r#type: 7,
            value_type: 0,
            interface_id: "0".to_string(),
            tags: vec![],
            delay: "60s".to_string(),
        };

        match &self.client.create_item(&self.session, &params) {
            Ok(item_id) => {
                self.latest_item_id = item_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("item create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_trigger(
        &mut self,
        host_name: &str,
        description: &str,
        item_key: &str,
    ) -> &mut Self {
        let expression = format!("last(/{host_name}/{item_key})=0");

        let params = CreateTriggerRequest {
            description: description.to_string(),
            expression: expression.to_string(),
            dependencies: vec![],
            tags: vec![],
        };

        match &self.client.create_trigger(&self.session, &params) {
            Ok(trigger_id) => {
                self.latest_trigger_id = trigger_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("trigger create error: {}", e);
                panic!("{}", e)
            }
        }
    }

    pub fn create_web_scenario(&mut self, name: &str) -> &mut Self {
        let step = ZabbixWebScenarioStep {
            name: "Check github.com page".to_string(),
            url: "https://github.com".to_string(),
            status_codes: "200".to_string(),
            no: "0".to_string(),
        };

        let request = CreateWebScenarioRequest {
            name: name.to_string(),
            host_id: self.latest_host_id.to_string(),
            steps: vec![step],
        };

        match &self.client.create_webscenario(&self.session, &request) {
            Ok(webscenario_id) => {
                self.latest_webscenario_id = webscenario_id.to_owned();
                self
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                error!("web-scenario create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

//mod integration;

#[test]
fn get_api_info() {
    if are_integration_tests_enabled() {
        let test_env = TestEnvBuilder::build();

        match test_env.client.get_api_info() {
            Ok(result) => {
                assert!(!result.is_empty())
            }
            Err(e) => {
                eprintln!("error: {}", e);
                panic!("unexpected error")
            }
        }
    }
}

#[test]
fn session_should_be_returned() {
    init_logging();

    if are_integration_tests_enabled() {
        let http_client = Client::new();

        let tests_config = get_integration_tests_config();

        let client = ZabbixApiV6Client::new(http_client, &tests_config.zabbix_api_url);

        match client.get_auth_session(
            &tests_config.zabbix_api_user,
            &tests_config.zabbix_api_password,
        ) {
            Ok(session) => assert!(session.len() > 0),
            Err(e) => {
                eprintln!("error: {}", e);
                panic!("unexpected error")
            }
        }
    }
}

#[test]
fn raw_api_call_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();
        test_env.get_session();

        #[derive(Serialize)]
        struct Params {
            pub filter: Filter,
        }

        #[derive(Serialize)]
        struct Filter {
            pub host: Vec<String>,
        }

        let params = Params {
            filter: Filter {
                host: vec!["Zabbix server".to_string()],
            },
        };

        match test_env.client.raw_api_call::<Params, Vec<ZabbixHost>>(
            &test_env.session,
            "host.get",
            &params,
        ) {
            Ok(response) => {
                let results = response.result.unwrap();
                info!("{:?}", results.first().unwrap());
                assert_eq!(1, results.len())
            }
            Err(e) => {
                eprintln!("api call error: {}", e);
                panic!("unexpected api call error")
            }
        }
    }
}

#[test]
fn get_host_groups_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let group_name2 = get_random_string();
        let group_name3 = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host_group(&group_name2)
            .create_host_group(&group_name3);

        #[derive(Serialize)]
        struct Filter {
            pub name: Vec<String>,
        }

        let request = GetHostGroupsRequest {
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            filter: Filter {
                name: vec![group_name2.to_string()],
            },
        };

        match test_env.client.get_host_groups(&test_env.session, &request) {
            Ok(host_groups) => {
                assert_eq!(host_groups.len(), 1);

                let host_group = host_groups.first().unwrap();

                assert_eq!(&host_group.name, &group_name2)
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("host group get error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn get_hosts_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name1 = get_random_string();
        let host_name2 = get_random_string();
        let host_name3 = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name1)
            .create_host(&host_name2)
            .create_host(&host_name3);

        #[derive(Serialize)]
        struct Filter {
            pub host: Vec<String>,
        }

        let request = GetHostsRequest {
            filter: Filter {
                host: vec![host_name2.to_string()],
            },
        };

        match test_env.client.get_hosts(&test_env.session, &request) {
            Ok(hosts) => {
                assert_eq!(hosts.len(), 1);

                let host = hosts.first().unwrap();

                assert_eq!(&host.host, &host_name2)
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("host get error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn get_items_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name1 = get_random_string();
        let host_name2 = get_random_string();
        let host_name3 = get_random_string();
        let item_name = get_random_string();
        let item_key = format!("test{}", get_random_string());

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name1)
            .create_host(&host_name2)
            .create_host(&host_name3)
            .create_item(&item_name, &item_key);

        #[derive(Serialize)]
        struct Search {
            pub key_: String,
        }

        let request = GetItemsRequest {
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            with_triggers: false,
            host_ids: test_env.latest_host_id.to_string(),
            search: Search {
                key_: item_key.to_string(),
            },
            sort_field: "name".to_string(),
        };

        match test_env.client.get_items(&test_env.session, &request) {
            Ok(items) => {
                assert_eq!(items.len(), 1);

                let item = items.first().unwrap();

                assert_eq!(&item.key_, &item_key)
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("host get error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn get_triggers_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();
        let item_name = get_random_string();
        let item_key = get_random_string();
        let trigger_description = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name)
            .create_item(&item_name, &item_key)
            .create_trigger(&host_name, &trigger_description, &item_key);

        let request = GetTriggerByIdRequest {
            trigger_ids: test_env.latest_trigger_id.to_string(),
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            select_functions: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
        };

        match test_env.client.get_triggers(&test_env.session, &request) {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                let result = results.first().unwrap();

                assert_eq!(&result.description, &trigger_description)
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("host get error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn get_webscenarios_test() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();
        let item_name = get_random_string();
        let item_key = get_random_string();
        let trigger_description = get_random_string();
        let webscenario_name = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name)
            .create_item(&item_name, &item_key)
            .create_trigger(&host_name, &trigger_description, &item_key)
            .create_web_scenario(&webscenario_name);

        let request = GetWebScenarioByIdRequest {
            output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            select_steps: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
            httptest_ids: test_env.latest_webscenario_id.to_string(),
        };

        match test_env
            .client
            .get_webscenarios(&test_env.session, &request)
        {
            Ok(results) => {
                assert_eq!(results.len(), 1);
                let result = results.first().unwrap();

                assert_eq!(&result.name, &webscenario_name)
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("host get error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn create_host_group_and_host() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name);

        assert!(test_env.latest_host_group_id > 0);
        assert!(test_env.latest_host_id > 0);
    }
}

#[test]
fn create_item() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name);

        let item_key = get_random_string();
        let item_name = get_random_string();

        let request = CreateItemRequest {
            key_: item_key,
            name: item_name,
            host_id: test_env.latest_host_id.to_string(),
            r#type: 7,
            value_type: 4,
            interface_id: "0".to_string(),
            tags: vec![],
            delay: "30s".to_string(),
        };

        match test_env.client.create_item(&test_env.session, &request) {
            Ok(item_id) => {
                assert!(item_id > 0);
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("item create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn create_trigger() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();

        let item_name = get_random_string();
        let item_key = format!("key{}", get_random_string());

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name)
            .create_item(&item_name, &item_key);

        let trigger_description = get_random_string();

        let expression = format!("last(/{host_name}/{item_key})=0");

        let request = CreateTriggerRequest {
            description: trigger_description,
            expression: expression.to_string(),
            dependencies: vec![],
            tags: vec![],
        };

        match test_env.client.create_trigger(&test_env.session, &request) {
            Ok(trigger_id) => assert!(trigger_id > 0),
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("trigger create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn create_web_scenario() {
    init_logging();

    if are_integration_tests_enabled() {
        let mut test_env = TestEnvBuilder::build();

        let group_name = get_random_string();
        let host_name = get_random_string();

        test_env
            .get_session()
            .create_host_group(&group_name)
            .create_host(&host_name);

        let web_scenario_name = get_random_string();

        let step = ZabbixWebScenarioStep {
            name: "Check github.com page".to_string(),
            url: "https://github.com".to_string(),
            status_codes: "200".to_string(),
            no: "0".to_string(),
        };

        let request = CreateWebScenarioRequest {
            name: web_scenario_name,
            host_id: test_env.latest_host_id.to_string(),
            steps: vec![step],
        };

        match test_env
            .client
            .create_webscenario(&test_env.session, &request)
        {
            Ok(web_scenario_id) => {
                assert!(web_scenario_id > 0);
            }
            Err(e) => {
                if let Some(inner_source) = e.source() {
                    println!("Caused by: {}", inner_source);
                }

                eprintln!("web-scenario create error: {}", e);
                panic!("{}", e)
            }
        }
    }
}

#[test]
fn hello_world_test() {
    let want = String::from("Hello, World!");
    let result = crate::functions::hello_world();
    assert_eq!(want, result);
}
#[test]
fn greeting_test() {
    let want = String::from("Hello, Rusty!");
    let name = String::from("Rusty");
    let result = crate::functions::greeting(name);
    assert_eq!(want, result);
}
