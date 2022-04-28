pub mod p2p {
    use std::{fmt::Display, str::FromStr};

    use reqwest::Url;
    use serde::Deserialize;
    use serde_with::{DeserializeFromStr, SerializeDisplay};

    #[derive(Debug, Default, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PeerInfo {
        pub announced_address: Option<PeerAddress>,
        pub application: String,
        pub version: String,
        pub platform: Option<String>,
        pub share_address: bool,
    }

    #[derive(
        Clone, Debug, Default, DeserializeFromStr, Eq, Hash, PartialEq, SerializeDisplay, sqlx::Type,
    )]
    #[sqlx(transparent)]
    pub struct PeerAddress(String);

    impl Display for PeerAddress {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl FromStr for PeerAddress {
        type Err = anyhow::Error;

        #[tracing::instrument(name = "models::P2P::PeerAddress.try_from()")]
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            // Remove any existing scheme by splitting on "://" if it exists and only taking the right half
            // or taking the base value if no "://" exists
            let value = value.split_once("://").unwrap_or(("", value)).1;

            // Parse value with dummy scheme to validate proper url format.
            // Don't use a real scheme here because `[Url::parse]` will output
            // that scheme's default port as "None" for the address.
            let url = Url::parse(&format!("dummyscheme://{}", value))?;

            let host = url
                .host()
                .ok_or_else(|| anyhow::anyhow!("invalid url: {}", value))?;
            let port = url.port().unwrap_or(8123);

            let address = format!("{}:{}", host, port);
            Ok(PeerAddress(address))
        }
    }

    #[derive(Debug)]
    pub struct BlockId;

    #[derive(Debug)]
    pub struct Transaction;

    #[derive(Debug)]
    pub struct ExchangeableBlock;
}
