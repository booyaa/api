// Copyright 2015 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use {
    CommandResult,
    Cpu, FsMount, Host, Netif, NetifStatus, NetifIPv4, NetifIPv6, Os,
    Providers,
    Result,
};
use command::CommandTarget;
use file::{FileTarget, FileOwner};
use host::HostTarget;
use package::PackageTarget;
use rustc_serialize::json;
use service::ServiceTarget;
use super::Target;
use zmq;

//
// Command
//

impl CommandTarget for Target {
    fn exec(host: &mut Host, cmd: &str) -> Result<CommandResult> {
        try!(host.send("command::exec", zmq::SNDMORE));
        try!(host.send(cmd, 0));
        try!(host.recv_header());

        let exit_code = try!(host.expect_recvmsg("exit_code", 1)).as_str().unwrap().parse::<i32>().unwrap();
        let stdout = try!(host.expect_recv("stdout", 2));
        let stderr = try!(host.expect_recv("stderr", 3));

        Ok(CommandResult {
            exit_code: exit_code,
            stdout: stdout,
            stderr: stderr,
        })
    }
}

//
// File
//

impl FileTarget for Target {
    fn file_is_file(host: &mut Host, path: &str) -> Result<bool> {
        try!(host.send("file::is_file", zmq::SNDMORE));
        try!(host.send(path, 0));

        try!(host.recv_header());
        let result = try!(host.expect_recv("is_file", 1));
        Ok(result == "1")
    }

    fn file_exists(host: &mut Host, path: &str) -> Result<bool> {
        try!(host.send("file::exists", zmq::SNDMORE));
        try!(host.send(path, 0));

        try!(host.recv_header());
        let result = try!(host.expect_recv("exists", 1));
        Ok(result == "1")
    }

    fn file_delete(host: &mut Host, path: &str) -> Result<()> {
        try!(host.send("file::delete", zmq::SNDMORE));
        try!(host.send(path, 0));
        try!(host.recv_header());
        Ok(())
    }

    fn file_mv(host: &mut Host, path: &str, new_path: &str) -> Result<()> {
        try!(host.send("file::mv", zmq::SNDMORE));
        try!(host.send(path, zmq::SNDMORE));
        try!(host.send(new_path, 0));
        try!(host.recv_header());
        Ok(())
    }

    fn file_copy(host: &mut Host, path: &str, new_path: &str) -> Result<()> {
        try!(host.send("file::copy", zmq::SNDMORE));
        try!(host.send(path, zmq::SNDMORE));
        try!(host.send(new_path, 0));
        try!(host.recv_header());
        Ok(())
    }

    fn file_get_owner(host: &mut Host, path: &str) -> Result<FileOwner> {
        try!(host.send("file::get_owner", zmq::SNDMORE));
        try!(host.send(path, 0));
        try!(host.recv_header());

        Ok(FileOwner {
            user_name: try!(host.expect_recv("user_name", 1)),
            user_uid: try!(host.expect_recv("user_uid", 2)).parse::<u64>().unwrap(),
            group_name: try!(host.expect_recv("group_name", 3)),
            group_gid: try!(host.expect_recv("group_gid", 4)).parse::<u64>().unwrap()
        })
    }

    fn file_set_owner(host: &mut Host, path: &str, user: &str, group: &str) -> Result<()> {
        try!(host.send("file::set_owner", zmq::SNDMORE));
        try!(host.send(path, zmq::SNDMORE));
        try!(host.send(user, zmq::SNDMORE));
        try!(host.send(group, 0));
        try!(host.recv_header());
        Ok(())
    }

    fn file_get_mode(host: &mut Host, path: &str) -> Result<u16> {
        try!(host.send("file::get_mode", zmq::SNDMORE));
        try!(host.send(path, 0));
        try!(host.recv_header());
        Ok(try!(host.expect_recv("mode", 1)).parse::<u16>().unwrap())
    }

    fn file_set_mode(host: &mut Host, path: &str, mode: u16) -> Result<()> {
        try!(host.send("file::set_mode", zmq::SNDMORE));
        try!(host.send(path, zmq::SNDMORE));
        try!(host.send(&mode.to_string(), 0));
        try!(host.recv_header());
        Ok(())
    }
}

//
// Host
//

impl HostTarget for Target {
    fn telemetry_cpu(host: &mut Host) -> Result<Cpu> {
        try!(host.send("telemetry::cpu", 0));
        try!(host.recv_header());

        let cpu = try!(host.expect_recv("cpu", 1));
        Ok(try!(json::decode(&cpu)))
    }

    fn telemetry_fs(host: &mut Host) -> Result<Vec<FsMount>> {
        try!(host.send("telemetry::fs", 0));
        try!(host.recv_header());

        let fs = try!(host.expect_recv("fs", 1));
        Ok(try!(json::decode(&fs)))
    }

    fn telemetry_net(host: &mut Host) -> Result<Vec<Netif>> {
        try!(host.send("telemetry::net", 0));
        try!(host.recv_header());

        let cpu = try!(host.expect_recv("net", 1));
        Ok(try!(json::decode(&cpu)))
    }

    fn telemetry_os(host: &mut Host) -> Result<Os> {
        try!(host.send("telemetry::os", 0));
        try!(host.recv_header());

        let cpu = try!(host.expect_recv("os", 1));
        Ok(try!(json::decode(&cpu)))
    }
}

//
// Package
//

impl PackageTarget for Target {
    fn default_provider(host: &mut Host) -> Result<Providers> {
        try!(host.send("package::default_provider", 0));
        try!(host.recv_header());

        let provider = try!(host.expect_recv("provider", 1));

        Ok(Providers::from(provider))
    }
}

//
// Service
//

impl ServiceTarget for Target {
    fn service_action(host: &mut Host, name: &str, action: &str) -> Result<CommandResult> {
        try!(host.send("service::action", zmq::SNDMORE));
        try!(host.send(name, zmq::SNDMORE));
        try!(host.send(action, 0));
        try!(host.recv_header());

        let exit_code = try!(host.expect_recvmsg("exit_code", 1)).as_str().unwrap().parse::<i32>().unwrap();
        let stdout = try!(host.expect_recv("stdout", 2));
        let stderr = try!(host.expect_recv("stderr", 3));

        Ok(CommandResult {
            exit_code: exit_code,
            stdout: stdout,
            stderr: stderr,
        })
    }
}
