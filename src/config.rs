#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub wp2l2d_host: String,
    pub wp2l2d_port: String,
    pub wp2l2d_cert_file: Option<String>,
    pub wp2l2d_key_file: Option<String>,
    pub wp_feed_url: String,
    pub line_native_country: String,
    pub line_pub_to_country: Option<String>,
    pub line_excl_from_country: Option<String>,
    pub line_lang: Option<String>,
}

pub fn create() -> Config {
    ::envy::from_env::<Config>().unwrap_or_else(|msg| {
        eprintln!(
            "{}.\nTry setting env variable for the missing field above (all caps).",
            msg
        );
        ::std::process::exit(-1)
    })
}
