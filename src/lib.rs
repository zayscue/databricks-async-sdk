#[derive(Debug, PartialEq)]
pub struct Config {
    client_id: String,
    secret: String,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    client_id: String,
    secret: String,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            client_id: String::from(""),
            secret: String::from(""),
        }
    }

    pub fn client_id(mut self, client_id: String) -> ConfigBuilder {
        self.client_id = client_id;
        self
    }

    pub fn secret(mut self, secret: String) -> ConfigBuilder {
        self.secret = secret;
        self
    }

    pub fn build(self) -> Config {
        Config {
            client_id: self.client_id,
            secret: self.secret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        // This is a test
        let config = Config {
            client_id: String::from("test-client-id"),
            secret: String::from("test-secret"),
        };

        let conf_from_builder: Config = ConfigBuilder::new()
            .client_id(String::from("test-client-id"))
            .secret(String::from("test-secret"))
            .build();

        assert_eq!(config, conf_from_builder)
    }
}
