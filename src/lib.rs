// Copyright (c) 2018, The rust-connman authors.

mod manager;

use manager::NetConnmanManager;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

extern crate dbus;

use dbus::arg;

const CONNMAN_DBUS_DEST: &'static str = "net.connman";

#[derive(Debug)]
struct Connection {
    pub connection: dbus::Connection,
    pub timeout_ms: i32,
}

#[derive(Debug)]
pub struct Manager {
    connection: Rc<RefCell<Connection>>,
}

pub struct Service {
    connection: Rc<RefCell<Connection>>,
    path: dbus::Path<'static>,
    info: HashMap<String, arg::Variant<Box<arg::RefArg>>>,
}

#[derive(Debug)]
pub enum Error {
    DBusError(dbus::Error),
    Other(String),
    KeyError(&'static str),
}

impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        Error::DBusError(e)
    }
}

impl Manager {
    pub fn new() -> Result<Manager, Error> {
        Ok(Manager {
            connection: Rc::new(RefCell::new(Connection {
                connection: dbus::Connection::get_private(dbus::BusType::System)?,
                timeout_ms: 50000,
            }))
        })
    }

    pub fn services(&mut self) -> Result<Vec<Service>, Error> {
        let services_array = self.connection.borrow_mut().connection.with_path(CONNMAN_DBUS_DEST, "/", 5000).get_services()?;
        let mut services: Vec<Service> = Vec::new();
        for (service_path, service_info) in services_array {
            services.push(Service::new(self.connection.clone(), service_path, service_info));
        }

        Ok(services)
    }
 }

impl Service {
    fn new(connection: Rc<RefCell<Connection>>, path: dbus::Path<'static>, info: HashMap<String, arg::Variant<Box<arg::RefArg>>>) -> Service {
        Service {
            connection: connection,
            path: path,
            info: info,
        }
    }

    pub fn name(&self) -> Result<&str, Error> {
        self.info.get("Name").and_then(|v| v.0.as_str()).ok_or(Error::KeyError("Name"))
    }
}
