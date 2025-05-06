use std::sync::Arc;

use otus_thiserror::smart::device::{SmartSocket, SmartThermometer};
use otus_thiserror::smart::{
    location::{SmartHouse, SmartRoom},
    report::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
};

#[cfg(test)]
mod tests {
    use custom::Robot;

    use super::*;

    #[test]
    fn construct_house() {
        let mut hell = SmartHouse::new("hell".to_string());
        let limb = SmartRoom::new("limb".to_string());
        let lust = SmartRoom::new("lust".to_string());

        assert!(hell.add(limb).is_ok(), "Limb should not be added before");
        assert!(hell.add(lust).is_ok(), "Lust should not be added before");

        let limb = SmartRoom::new("limb".to_string());
        assert!(hell.add(limb).is_err(), "Limb has already been added");

        hell.del("limb");

        assert!(hell.get_rooms().eq(&[SmartRoom::new("lust".to_string())]),);
    }

    #[test]
    fn plug_devices() {
        let mut boiler = SmartRoom::new("Boiler".to_string());

        let thermo = Arc::new(SmartThermometer::new("Thermometer 1".to_string()));
        let socket = Arc::new(SmartSocket::new("Main socket".to_string()));

        assert!(
            boiler.plug(thermo).is_ok(),
            "Thermometer successfully connected"
        );
        assert!(boiler.plug(socket).is_ok(), "Socket successfully connected");

        let socket = Arc::new(SmartSocket::new("Main socket".to_string()));
        assert!(boiler.plug(socket).is_err(), "Socket already connected");

        boiler.unplug("Main socket");

        assert!(boiler.devices().eq(&["Thermometer 1"]));
    }

    #[test]
    fn report_owning_report_provider() {
        let mut house = SmartHouse::new("Sweet home".to_string());
        let mut boiler = SmartRoom::new("Boiler".to_string());

        let thermo = SmartThermometer::new("Main thermometer".to_string());
        let socket = SmartSocket::new("Main socket".to_string());

        let _ = boiler.plug(Arc::new(thermo.clone()));
        let _ = boiler.plug(Arc::new(socket.clone()));

        let _ = house.add(boiler);

        let report_provider = OwningDeviceInfoProvider { socket };

        if let Ok(report) = house.create_report(report_provider) {
            let expected = "-> House: Sweet home
 --> Room: Boiler
 ----> Device: Socket[Main socket]
"
            .to_string();

            assert_eq!(report, expected, "Compare got and expected reports");
        } else {
            panic!("Houston we have a problem")
        }
    }

    #[test]
    fn report_borrowing_report_provider() {
        let mut house = SmartHouse::new("Millennium Falcon".to_string());
        let mut wheelhouse = SmartRoom::new("Wheelhouse".to_string());

        let thermo = SmartThermometer::new("Main thermometer".to_string());
        let socket = SmartSocket::new("Main socket".to_string());

        let _ = wheelhouse.plug(Arc::new(thermo.clone()));
        let _ = wheelhouse.plug(Arc::new(socket.clone()));

        let _ = house.add(wheelhouse);

        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };

        if let Ok(report) = house.create_report(info_provider) {
            let expected = "-> House: Millennium Falcon
 --> Room: Wheelhouse
 ----> Device: Socket[Main socket]
 ----> Device: Thermometer[Main thermometer]
"
            .to_string();

            assert_eq!(report, expected, "Compare got and expected reports");
        } else {
            panic!("Houston we have a problem");
        }
    }

    #[test]
    fn test_devices() {
        let mut habitacion = SmartRoom::new("Planta uno".to_string());

        let thermo = Arc::new(SmartThermometer::new("Main thermometer".to_string()));
        let socket = Arc::new(SmartSocket::new("Main socket".to_string()));
        let robot = Arc::new(Robot::new("Alice".to_string()));

        let _ = habitacion.plug(thermo.clone());
        let _ = habitacion.plug(socket.clone());
        let _ = habitacion.plug(robot.clone());

        assert_eq!(
            habitacion.devices(),
            vec!["Main thermometer", "Main socket", "Alice"]
        );
    }

    pub mod custom {
        use otus_thiserror::smart::{Named, Pluggable};
        pub struct Robot {
            name: String,
        }

        impl Robot {
            pub fn new(name: String) -> Self {
                Self { name }
            }
        }

        impl Named for Robot {
            fn name(&self) -> &str {
                &self.name
            }
        }

        impl Pluggable for Robot {}
    }
}
