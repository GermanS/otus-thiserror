use core::fmt;
use std::{error::Error, sync::Arc};

use crate::smart::Pluggable;
use crate::smart::Reportable;

#[derive(Clone)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<SmartRoom>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add(&mut self, room: SmartRoom) -> Result<(), Box<dyn Error>> {
        match self.get_rooms().iter().find(|&v| v.name() == room.name()) {
            Some(_) => Err(format!("room {} already constructed", room.name()).into()),
            None => {
                self.rooms.push(room);

                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn del(&mut self, room: &str) {
        if let Some(index) = self.get_rooms().iter().position(|r| r.name() == room) {
            self.rooms.remove(index);
        }
    }

    pub fn get_rooms(&self) -> &[SmartRoom] {
        &self.rooms
    }
    pub fn create_report<T: Reportable>(&self, report: T) -> Result<String, Box<dyn Error>> {
        report.make(self)
    }
}

impl PartialEq for SmartRoom {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl fmt::Display for SmartHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-> House: {}", self.name())
    }
}

#[derive(Clone)]
pub struct SmartRoom {
    name: String,
    devices: Vec<Arc<dyn Pluggable>>,
}

impl SmartRoom {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::default(),
        }
    }
    pub fn plug(&mut self, device: Arc<dyn Pluggable>) -> Result<(), Box<dyn Error>> {
        match &self.devices.iter().find(|&d| d.name() == device.name()) {
            Some(_) => Err(format!("Device with name {} already pluged", device.name()).into()),
            None => {
                self.devices.push(device);
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn unplug(&mut self, device: &str) {
        if let Some(index) = self.devices.iter().position(|d| d.name() == device) {
            self.devices.remove(index);
        }
    }

    pub fn is_connected(&self, device: &dyn Pluggable) -> bool {
        self.devices.iter().any(|d| d.name() == device.name())
    }

    #[allow(dead_code)]
    pub fn devices(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.name().to_string()).collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for SmartRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--> Room: {}", self.name())
    }
}
