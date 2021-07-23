use std::env;

use ethsign::*;
use rustc_hex::FromHex;

use ya_core_model::NodeId;

use crate::id_key::IdentityKey;

// autoconfiguration
const ENV_AUTOCONF_PK: &str = "YAGNA_AC_IDENTITY_PK";
const ENV_AUTOCONF_APP_KEY: &str = "YAGNA_AC_APPKEY";

pub fn preconfigured_identity(password: Protected) -> anyhow::Result<Option<IdentityKey>> {
    let secret_hex: Vec<u8> = match env::var(ENV_AUTOCONF_PK) {
        Ok(v) => v.from_hex()?,
        Err(_) => return Ok(None),
    };
    let secret = SecretKey::from_raw(&secret_hex)?;
    Ok(Some(IdentityKey::from_secret(None, secret, password)))
}

pub fn preconfigured_node_id() -> anyhow::Result<Option<NodeId>> {
    let secret_hex: Vec<u8> = match env::var(ENV_AUTOCONF_PK) {
        Ok(v) => v.from_hex()?,
        Err(_) => return Ok(None),
    };
    let secret = SecretKey::from_raw(&secret_hex)?;
    Ok(Some(NodeId::from(secret.public().address().as_ref())))
}

pub fn preconfigured_appkey() -> anyhow::Result<Option<String>> {
    Ok(env::var(ENV_AUTOCONF_APP_KEY).ok())
}
