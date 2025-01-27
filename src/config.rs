use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub host: wp2l2d,
    pub port: wp2l2d,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub wp_feed_url: https://www.cilacap.info/feed,
    pub line_native_country: ID,
    pub line_pub_to_country: id,
    pub line_excl_from_country: ID,
    pub line_lang: id,
    pub publish_duration_in_weeks: 1,
}

pub fn create() -> Config {
    envy::from_env::<Config>().unwrap_or_else(|err| {
        match err {
            envy::Error::MissingValue(v) => eprintln!(
                "Environment variable '{}' is not set.",
                v.to_string().to_uppercase()
            ),
            _ => eprintln!("Error when parsing environment variables: {}", err),
        };
        std::process::exit(-1)
    })
}
