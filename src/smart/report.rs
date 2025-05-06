use std::error::Error;

use super::device::{SmartSocket, SmartThermometer};
use super::location::SmartHouse;

pub trait Reportable {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>>;
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl Reportable for BorrowingDeviceInfoProvider<'_, '_> {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>> {
        let mut plugged_socket_room = None;
        let mut plugged_thermo_room = None;

        for room in house.get_rooms().iter() {
            if room.is_connected(self.socket) {
                plugged_socket_room = Some(room);
            }

            if room.is_connected(self.thermo) {
                plugged_thermo_room = Some(room);
            }
        }

        if plugged_thermo_room.is_none() && plugged_socket_room.is_none() {
            return Err("Devices not found".into());
        }

        let mut out;

        if plugged_socket_room.is_some() && plugged_thermo_room.is_some() {
            let plugged_socket_room = plugged_socket_room.unwrap();
            let plugged_thermo_room = plugged_thermo_room.unwrap();

            if plugged_socket_room.name() == plugged_thermo_room.name() {
                out = format!(
                    "{} {} {} {}",
                    house, plugged_socket_room, self.socket, self.thermo
                );
            } else {
                out = format!(
                    "{} {} {} {} {}",
                    house, plugged_socket_room, self.socket, plugged_thermo_room, self.thermo
                );
            }
        } else {
            match plugged_socket_room.is_some() {
                true => {
                    out = format!("{} {} {}", house, plugged_socket_room.unwrap(), self.socket);
                }
                false => {
                    out = format!("not found {}", self.socket);
                }
            };

            match plugged_thermo_room.is_some() {
                true => {
                    out = format!(
                        "{}\n {} {} {}",
                        out,
                        house,
                        plugged_thermo_room.unwrap(),
                        self.thermo
                    );
                }
                false => {
                    out = format!("{} not found {}", out, self.thermo);
                }
            }
        }

        Ok(out)
    }
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

impl Reportable for OwningDeviceInfoProvider {
    fn make(&self, house: &SmartHouse) -> Result<String, Box<dyn Error>> {
        for room in house.get_rooms().iter() {
            if room.is_connected(&self.socket) {
                let out = format!("{} {} {}", house, room, &self.socket);

                return Ok(out);
            }
        }

        Err("Device not found".into())
    }
}
