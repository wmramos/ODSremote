use base::config::keys;
use hbb_common::config;

const QUICK_SUPPORT: bool = cfg!(feature = "odsremote-qs");
const SERVER_SETTINGS: [(&str, &str); 4] = [
    (keys::OPTION_CUSTOM_RENDEZVOUS_SERVER, "rust.onedotsystems.com:21116"),
    (keys::OPTION_RELAY_SERVER, "rust.onedotsystems.com:21117"),
    (keys::OPTION_API_SERVER, "https://rust.onedotsystems.com"),
    (keys::OPTION_KEY, "VrszR71y9BuwelF2PS6rTNeP6fRKkRq8FqoucT3YZdI="),
];

pub fn initialize() {
    // A separate identity keeps portable sessions out of the installed Agent's config and IPC.
    *config::APP_NAME.write().unwrap() = if QUICK_SUPPORT { "ODSremoteQS" } else { "ODSremote" }.to_owned();
    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    for (key, value) in [
        (keys::OPTION_APPROVE_MODE, "click"),
        (keys::OPTION_ALLOW_AUTO_UPDATE, "N"),
        (keys::OPTION_ALLOW_WEBSOCKET, "N"),
        (keys::OPTION_DISABLE_UDP, "N"),
        (keys::OPTION_ENABLE_TCP_PUNCH, "Y"),
        (keys::OPTION_ENABLE_UDP_PUNCH, "N"),
        (keys::OPTION_ENABLE_IPV6_PUNCH, "N"),
        (keys::OPTION_ENABLE_WEBRTC, "N"),
    ] {
        defaults.insert(key.to_owned(), value.to_owned());
    }
    drop(defaults);
    let mut overrides = config::OVERWRITE_SETTINGS.write().unwrap();
    for (key, value) in SERVER_SETTINGS {
        overrides.insert(key.to_owned(), value.to_owned());
    }
    // Never fetch an upstream unbranded executable as an ODSremote update.
    overrides.insert(keys::OPTION_ALLOW_AUTO_UPDATE.to_owned(), "N".to_owned());
    if QUICK_SUPPORT {
        overrides.insert(keys::OPTION_APPROVE_MODE.to_owned(), "click".to_owned());
        overrides.insert(keys::OPTION_VERIFICATION_METHOD.to_owned(), "use-temporary-password".to_owned());
        config::BUILTIN_SETTINGS.write().unwrap().insert(keys::OPTION_ALLOW_LOGON_SCREEN_PASSWORD.to_owned(), "N".to_owned());
        let mut hard = config::HARD_SETTINGS.write().unwrap();
        for (key, value) in [
            ("conn-type", "incoming"),
            ("disable-installation", "Y"),
            ("disable-account", "Y"),
            ("disable-ab", "Y"),
        ] {
            hard.insert(key.to_owned(), value.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branded_profile_uses_public_server_settings_without_credentials() {
        assert_eq!(SERVER_SETTINGS.len(), 4);
        assert!(SERVER_SETTINGS.iter().all(|(key, _)| !key.contains("password") && !key.contains("token")));
        assert_eq!(SERVER_SETTINGS[3].1, "VrszR71y9BuwelF2PS6rTNeP6fRKkRq8FqoucT3YZdI=");
    }

    #[test]
    fn edition_enforces_its_approval_and_installation_policy() {
        initialize();
        assert_eq!(config::Config::get_option(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER), SERVER_SETTINGS[0].1);
        assert_eq!(config::Config::get_option(keys::OPTION_ALLOW_AUTO_UPDATE), "N");
        assert_eq!(config::is_disable_installation(), QUICK_SUPPORT);
        if QUICK_SUPPORT {
            assert_eq!(config::Config::get_option(keys::OPTION_APPROVE_MODE), "click");
            assert_eq!(config::Config::get_option(keys::OPTION_VERIFICATION_METHOD), "use-temporary-password");
            assert_eq!(crate::common::get_uri_prefix(), "odsremoteqs://");
        } else {
            assert_eq!(crate::common::get_uri_prefix(), "odsremote://");
        }
    }
}
