// Copyright 2020 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::config::Config;
use clap::ArgMatches;

pub(crate) mod init;
pub(crate) mod run;
pub(crate) mod unregister;
pub(crate) mod upgrade;

pub(crate) fn override_config(mut config: Config, matches: &ArgMatches) -> Config {
    let mut was_mix_host_overridden = false;
    if let Some(mix_host) = matches.value_of("mix-host") {
        config = config.with_mix_listening_host(mix_host);
        was_mix_host_overridden = true;
    }

    if let Some(mix_port) = matches.value_of("mix-port").map(|port| port.parse::<u16>()) {
        if let Err(err) = mix_port {
            // if port was overridden, it must be parsable
            panic!("Invalid port value provided - {:?}", err);
        }
        config = config.with_mix_listening_port(mix_port.unwrap());
    }
    let mut was_clients_host_overridden = false;
    if let Some(clients_host) = matches.value_of("clients-host") {
        config = config.with_clients_listening_host(clients_host);
        was_clients_host_overridden = true;
    }

    if let Some(clients_port) = matches
        .value_of("clients-port")
        .map(|port| port.parse::<u16>())
    {
        if let Err(err) = clients_port {
            // if port was overridden, it must be parsable
            panic!("Invalid port value provided - {:?}", err);
        }
        config = config.with_clients_listening_port(clients_port.unwrap());
    }

    if let Some(mix_announce_host) = matches.value_of("mix-announce-host") {
        config = config.with_mix_announce_host(mix_announce_host);
    } else if was_mix_host_overridden {
        // make sure our 'mix-announce-host' always defaults to 'mix-host'
        config = config.mix_announce_host_from_listening_host()
    }

    if let Some(mix_announce_port) = matches
        .value_of("mix-announce-port")
        .map(|port| port.parse::<u16>())
    {
        if let Err(err) = mix_announce_port {
            // if port was overridden, it must be parsable
            panic!("Invalid port value provided - {:?}", err);
        }
        config = config.with_mix_announce_port(mix_announce_port.unwrap());
    }

    if let Some(clients_announce_host) = matches.value_of("clients-announce-host") {
        config = config.with_clients_announce_host(clients_announce_host);
    } else if was_clients_host_overridden {
        // make sure our 'clients-announce-host' always defaults to 'clients-host'
        config = config.clients_announce_host_from_listening_host()
    }

    if let Some(clients_announce_port) = matches
        .value_of("clients-announce-port")
        .map(|port| port.parse::<u16>())
    {
        if let Err(err) = clients_announce_port {
            // if port was overridden, it must be parsable
            panic!("Invalid port value provided - {:?}", err);
        }
        config = config.with_clients_announce_port(clients_announce_port.unwrap());
    }

    if let Some(validator) = matches.value_of("validator") {
        config = config.with_custom_validator(validator);
    }

    if let Some(inboxes_dir) = matches.value_of("inboxes") {
        config = config.with_custom_clients_inboxes(inboxes_dir);
    }

    if let Some(clients_ledger) = matches.value_of("clients-ledger") {
        config = config.with_custom_clients_ledger(clients_ledger);
    }

    if let Some(location) = matches.value_of("location") {
        config = config.with_location(location);
    }

    if let Some(incentives_address) = matches.value_of("incentives-address") {
        config = config.with_incentives_address(incentives_address);
    }

    config
}
