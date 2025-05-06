use core::fmt;

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Pluggable: Named {}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    name: String,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Named for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pluggable for SmartSocket {}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----> Device: Socket[{}]", self.name())
    }
}

#[derive(Clone)]
pub struct SmartThermometer {
    name: String,
}

impl SmartThermometer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Named for SmartThermometer {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pluggable for SmartThermometer {}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----> Device: Thermometer[{}]", self.name())
    }
}
