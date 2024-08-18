use std::fmt::Display;

use super::Unit;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum MetricUnitVariant {
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
}

impl From<&str> for MetricUnitVariant {
    fn from(s: &str) -> Self {
        match s {
            "mm" => Self::Millimeter,
            "cm" => Self::Centimeter,
            "dm" => Self::Decimeter,
            "m" => Self::Meter,
            _ => panic!("Unknown unit")
        }
    }
}

impl From<&MetricUnitVariant> for String { 
    fn from(unit: &MetricUnitVariant) -> Self {
        match unit {
            MetricUnitVariant::Millimeter => String::from("mm"),
            MetricUnitVariant::Centimeter => String::from("cm"),
            MetricUnitVariant::Decimeter => String::from("dm"),
            MetricUnitVariant::Meter => String::from("m"),
        }
    }
}

impl Display for MetricUnitVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MetricUnit {
    value: f64,
    unit: MetricUnitVariant
}

impl MetricUnit {
    pub fn new(value: f64, unit: MetricUnitVariant) -> Self {
        Self {
            value,
            unit
        }
    }
}

impl Unit for MetricUnit {
    type UnitType = MetricUnitVariant;
    
    fn unit(&self) -> Self::UnitType {
        self.unit
    }
    
    fn grow_type(&mut self) {
        match self.unit {
            MetricUnitVariant::Meter => {},
            MetricUnitVariant::Decimeter => {self.value /= 10.0; self.unit = MetricUnitVariant::Meter;},
            MetricUnitVariant::Centimeter => {self.value /= 10.0; self.unit = MetricUnitVariant::Decimeter;},
            MetricUnitVariant::Millimeter => {self.value /= 10.0; self.unit = MetricUnitVariant::Centimeter;},
        }
    }
    
    fn shrink_type(&mut self) {
        match self.unit {
            MetricUnitVariant::Meter => {self.value *= 10.0; self.unit = MetricUnitVariant::Decimeter;},
            MetricUnitVariant::Decimeter => {self.value *= 10.0; self.unit = MetricUnitVariant::Centimeter;},
            MetricUnitVariant::Centimeter => {self.value *= 10.0; self.unit = MetricUnitVariant::Millimeter;},
            MetricUnitVariant::Millimeter => {},
        }
    }
}

impl Display for MetricUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}