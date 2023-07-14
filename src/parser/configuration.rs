use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub blue_team: TeamConfiguration,
    pub red_team: TeamConfiguration,
}

#[derive(Deserialize, Clone)]
pub struct TeamConfiguration {
    pub network_configuration: NetworkConfiguration,
}

#[derive(Deserialize, Clone)]
pub struct NetworkConfiguration {
    pub bind_address: String,
    pub listen_address: String,
}