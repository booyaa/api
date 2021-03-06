// Copyright 2015-2017 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use command::{CommandResult, CommandTarget};
use directory::DirectoryTarget;
use error::{Error, Result};
use file::{FileTarget, FileOwner};
use host::Host;
use package::PackageTarget;
use package::providers::Providers;
use serde_json;
use service::ServiceTarget;
use std::{env, process, str};
use std::path::Path;
use super::{default_base as default, linux_base as linux};
use host::telemetry::{Cpu, Os, Telemetry, TelemetryTarget};

pub struct NixOsTarget;

//
// Command
//

impl CommandTarget for NixOsTarget {
    #[allow(unused_variables)]
    fn exec(host: &mut Host, cmd: &str) -> Result<CommandResult> {
        default::command_exec(cmd)
    }
}

//
// Directory
//

impl<P: AsRef<Path>> DirectoryTarget<P> for NixOsTarget {
    #[allow(unused_variables)]
    fn directory_is_directory(host: &mut Host, path: P) -> Result<bool> {
        default::directory_is_directory(path)
    }

    #[allow(unused_variables)]
    fn directory_exists(host: &mut Host, path: P) -> Result<bool> {
        default::file_exists(path)
    }

    #[allow(unused_variables)]
    fn directory_create(host: &mut Host, path: P, recursive: bool) -> Result<()> {
        default::directory_create(path, recursive)
    }

    #[allow(unused_variables)]
    fn directory_delete(host: &mut Host, path: P, recursive: bool) -> Result<()> {
        default::directory_delete(path, recursive)
    }

    #[allow(unused_variables)]
    fn directory_mv(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_mv(path, new_path)
    }

    #[allow(unused_variables)]
    fn directory_get_owner(host: &mut Host, path: P) -> Result<FileOwner> {
        linux::file_get_owner(path)
    }

    #[allow(unused_variables)]
    fn directory_set_owner(host: &mut Host, path: P, user: &str, group: &str) -> Result<()> {
        default::file_set_owner(path, user, group)
    }

    #[allow(unused_variables)]
    fn directory_get_mode(host: &mut Host, path: P) -> Result<u16> {
        linux::file_get_mode(path)
    }

    #[allow(unused_variables)]
    fn directory_set_mode(host: &mut Host, path: P, mode: u16) -> Result<()> {
        default::file_set_mode(path, mode)
    }
}

//
// File
//

impl<P: AsRef<Path>> FileTarget<P> for NixOsTarget {
    #[allow(unused_variables)]
    fn file_is_file(host: &mut Host, path: P) -> Result<bool> {
        default::file_is_file(path)
    }

    #[allow(unused_variables)]
    fn file_exists(host: &mut Host, path: P) -> Result<bool> {
        default::file_exists(path)
    }

    #[allow(unused_variables)]
    fn file_delete(host: &mut Host, path: P) -> Result<()> {
        default::file_delete(path)
    }

    #[allow(unused_variables)]
    fn file_mv(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_mv(path, new_path)
    }

    #[allow(unused_variables)]
    fn file_copy(host: &mut Host, path: P, new_path: P) -> Result<()> {
        default::file_copy(path, new_path)
    }

    #[allow(unused_variables)]
    fn file_get_owner(host: &mut Host, path: P) -> Result<FileOwner> {
        linux::file_get_owner(path)
    }

    #[allow(unused_variables)]
    fn file_set_owner(host: &mut Host, path: P, user: &str, group: &str) -> Result<()> {
        default::file_set_owner(path, user, group)
    }

    #[allow(unused_variables)]
    fn file_get_mode(host: &mut Host, path: P) -> Result<u16> {
        linux::file_get_mode(path)
    }

    #[allow(unused_variables)]
    fn file_set_mode(host: &mut Host, path: P, mode: u16) -> Result<()> {
        default::file_set_mode(path, mode)
    }
}

//
// Package
//

impl PackageTarget for NixOsTarget {
    fn default_provider(host: &mut Host) -> Result<Providers> {
        default::default_provider(host, vec![Providers::Nix])
    }
}

//
// Service
//

impl ServiceTarget for NixOsTarget {
    #[allow(unused_variables)]
    fn service_action(host: &mut Host, name: &str, action: &str) -> Result<Option<CommandResult>> {
        linux::service_systemd(name, action)
    }
}

//
// Telemetry
//

impl TelemetryTarget for NixOsTarget {
    #[allow(unused_variables)]
    fn telemetry_init(host: &mut Host) -> Result<serde_json::Value> {
        let cpu_vendor = try!(linux::cpu_vendor());
        let cpu_brand = try!(linux::cpu_brand_string());
        let hostname = try!(default::hostname());
        let (version, maj, min, patch) = try!(version());

        let telemetry = Telemetry::new(
            Cpu::new(
                &cpu_vendor,
                &cpu_brand,
                try!(linux::cpu_cores()),
            ),
            try!(default::fs()),
            &hostname,
            try!(linux::memory()),
            default::net(),
            Os::new(
                env::consts::ARCH,
                "linux",
                "nixos",
                &version,
                maj,
                min,
                patch
            ),
        );

        Ok(serde_json::to_value(telemetry)?)
    }
}

fn version() -> Result<(String, u32, u32, u32)> {
    let out = process::Command::new("nixos-version").output()?;
    let version_str = str::from_utf8(&out.stdout).or(Err(Error::Generic("Could not read OS version".into())))?.trim();
    let mut parts = version_str.split('.');
    let version_maj = parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32.hash (codename)`. Got: {}", version_str)))?.parse()?;
    let version_min = parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32.hash (codename)`. Got: {}", version_str)))?.parse()?;
    let version_patch = parts.next().ok_or(Error::Generic(format!("Expected OS version format `u32.u32.u32.hash (codename)`. Got: {}", version_str)))?.parse()?;
    Ok((version_str.into(), version_maj, version_min, version_patch))
}
