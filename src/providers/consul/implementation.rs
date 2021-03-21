use consul::{
    Client,
    Config,
    health::Health,
};
use reqwest::ClientBuilder;
use super::{
    conf::{
        consul_url,
        consul_service,
    },
    models::ProviderInner,
};

pub(crate) fn providers() -> Result<Vec<ProviderInner>, &'static str> {
    let config = ClientBuilder::new()
        .build()
        .map_err(|_| "Failed to build reqwest client")
        .map(|client| Config {
            address: consul_url(),
            datacenter: None,
            http_client: client,
            wait_time: None,
        })?;
    let client = Client::new(config);
    let list = client.service(&consul_service(), None, true, None)
        .map_err(|_| "unable to get services")?.0
        .into_iter()
        .map(|res| ProviderInner {
            name: res.Service.ID,
            oauth: res.Service.Tags.map(|v| v.contains(&"oauth".to_string())).unwrap_or(false),
            addr: res.Service.Address,
            port: res.Service.Port,
        })
        .collect();
    Ok(list)
}