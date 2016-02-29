// Copyright 2015 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! FFI interface for Service

use {DataItem, Host};
use command::ffi::Ffi__CommandResult;
use host::ffi::Ffi__Host;
use libc::c_char;
use std::{convert, ptr, str};
use std::ffi::{CStr, CString};
use super::*;

#[repr(C)]
pub struct Ffi__Service {
    name: DataItem<*mut c_char>,
}

impl <'a>convert::From<Service<'a>> for Ffi__Service {
    fn from(service: Service) -> Ffi__Service {
        Ffi__Service {
            name: DataItem::from(service.name),
        }
    }
}

impl <'a>convert::From<Ffi__Service> for Service<'a> {
    fn from(ffi_service: Ffi__Service) -> Service<'a> {
        Service {
            name: DataItem::from(ffi_service.name),
        }
    }
}

#[no_mangle]
pub extern "C" fn service_new(name_ptr: *const DataItem<*mut c_char>) -> Ffi__Service {
    let name = DataItem::from(unsafe { ptr::read(name_ptr) });
    Ffi__Service::from(Service::new(name))
}

#[no_mangle]
pub extern "C" fn service_action(ffi_service_ptr: *const Ffi__Service, ffi_host_ptr: *const Ffi__Host, action_ptr: *const c_char) -> Ffi__CommandResult {
    let service = Service::from(unsafe { ptr::read(ffi_service_ptr) });
    let mut host = Host::from(unsafe { ptr::read(ffi_host_ptr) });
    let action = unsafe { str::from_utf8(CStr::from_ptr(action_ptr).to_bytes()).unwrap() };

    let result = Ffi__CommandResult::from(service.action(&mut host, action).unwrap());

    // Convert ZMQ socket to raw to avoid destructor closing sock
    Ffi__Host::from(host);

    result
}

#[cfg(test)]
mod tests {
    use {DataItem, DataOption, OsPlatform, Targets};
    #[cfg(feature = "remote-run")]
    use Host;
    use host::ffi::Ffi__Host;
    use libc::c_char;
    use Service;
    use std::ffi::{CStr, CString};
    use std::str;
    #[cfg(feature = "remote-run")]
    use std::thread;
    use super::*;
    #[cfg(feature = "remote-run")]
    use zmq;

    #[test]
    fn test_convert_service() {
        let service = Service {
            name: DataItem::new(vec![ DataOption::new(Targets::Default, "nginx") ]),
        };
        Ffi__Service::from(service);
    }

    #[test]
    fn test_convert_ffi_service() {
        let ffi_service = Ffi__Service {
            name: DataItem::new(vec![ DataOption::new(Targets::Default, CString::new("nginx").unwrap().into_raw()) ]),
        };
        Service::from(ffi_service);
    }

    #[test]
    fn test_service_new() {
        let name = DataItem::new(vec![ DataOption::new(Targets::Default, CString::new("nginx").unwrap().into_raw()) ]);
        let ffi_service = service_new(&name as *const DataItem<*mut c_char>);
        let result_name = ffi_service.name.resolve(&OsPlatform::Freebsd).unwrap();
        assert_eq!(str::from_utf8(unsafe { CStr::from_ptr(*result_name) }.to_bytes()).unwrap(), "nginx");
    }

    // XXX This requires mocking the shell or Command struct
    // #[cfg(feature = "local-run")]
    // #[test]
    // fn test_service_action() {
    // }

    #[cfg(feature = "remote-run")]
    #[test]
    fn test_service_action() {
        let mut ctx = zmq::Context::new();

        let mut agent_sock = ctx.socket(zmq::REP).unwrap();
        agent_sock.bind("inproc://test_action").unwrap();

        let agent_mock = thread::spawn(move || {
            assert_eq!("service::action", agent_sock.recv_string(0).unwrap().unwrap());
            assert_eq!(agent_sock.get_rcvmore().unwrap(), true);
            assert_eq!("nginx", agent_sock.recv_string(0).unwrap().unwrap());
            assert_eq!(agent_sock.get_rcvmore().unwrap(), true);
            assert_eq!("start", agent_sock.recv_string(0).unwrap().unwrap());
            assert_eq!(agent_sock.get_rcvmore().unwrap(), false);

            agent_sock.send_str("Ok", zmq::SNDMORE).unwrap();
            agent_sock.send_str("0", zmq::SNDMORE).unwrap();
            agent_sock.send_str("Service started...", zmq::SNDMORE).unwrap();
            agent_sock.send_str("", 0).unwrap();
        });

        let mut sock = ctx.socket(zmq::REQ).unwrap();
        sock.connect("inproc://test_action").unwrap();

        let ffi_host = Ffi__Host::from(Host::test_new(None, Some(sock), None, None));

        let name = DataItem::new(vec![ DataOption::new(Targets::Default, CString::new("nginx").unwrap().into_raw()) ]);
        let action = CString::new("start").unwrap().into_raw();

        let ffi_service = service_new(&name as *const DataItem<*mut c_char>);
        let result = service_action(&ffi_service as *const Ffi__Service, &ffi_host as *const Ffi__Host, action);

        assert_eq!(result.exit_code, 0);

        let stdout = unsafe { str::from_utf8(CStr::from_ptr(result.stdout).to_bytes()).unwrap() };
        assert_eq!(stdout, "Service started...");

        let stderr = unsafe { str::from_utf8(CStr::from_ptr(result.stderr).to_bytes()).unwrap() };
        assert_eq!(stderr, "");

        Host::from(ffi_host);

        agent_mock.join().unwrap();
    }
}
