use num_complex::Complex;
use std::fmt;

pub type DimensionVector = [i8; 8];

// ValueWithUnit struktur
// Representerar ett fysiskt värde med dimensioner, utan presentation
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,   // Komplext värde för att hantera fasrelationer
    pub dimension: DimensionVector,    // Dimensionsvektor [längd, tid, massa, ström, temp, substans, ljus, ortogonalitet]
    // Notera att unit_symbol har tagits bort enligt Const_presentation-designen
}

// QuantityError enum
// Omfattande felhantering för beräkningar med fysikaliska enheter
#[derive(Debug, Clone)]
pub enum QuantityError {
    IncompatibleDimensions {
        left_dim: DimensionVector,
        right_dim: DimensionVector,
        message: String,  // T.ex. "Kan inte addera 5 m och 3 s"
    },
    
    DimensionOverflow {
        dimension_index: usize,
        attempted_value: i16,
        message: String,  // T.ex. "Dimensionsöverflöde: exponent 130 för längd överstiger max 127"
    },
    
    NumericError {
        message: String,  // T.ex. "Värdeöverlopp vid multiplikation"
    },
    
    OrthogonalityError {
        left_ortho: i8,
        right_ortho: i8,
        message: String,  // T.ex. "Ortogonalitetsfel: kan inte kombinera energi och vridmoment"
    },
    
    DivisionByZero,  // Enkelt, behöver ingen extra information
    
    ConversionError {
        from_unit: String,
        to_unit: String,
        message: String,  // T.ex. "Kan inte konvertera från meter till sekunder"
    },
}

// Implementera grundläggande funktionalitet för ValueWithUnit
impl ValueWithUnit {
    // Skapa ett nytt reellt värde med enheter
    pub const fn new(value: f64, dimension: DimensionVector) -> Self {
        ValueWithUnit {
            value: Complex::new(value, 0.0),
            dimension,
        }
    }
    
    // Skapa ett nytt komplext värde med enheter
    pub const fn new_complex(re: f64, im: f64, dimension: DimensionVector) -> Self {
        ValueWithUnit {
            value: Complex::new(re, im),
            dimension,
        }
    }
}


// Implementera Display för ValueWithUnit
// Detta hanterar endast debugging och default-visning
// Den riktiga presentationen kommer hanteras separat
impl fmt::Display for ValueWithUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Enkel implementation som visar värdet och dimensionsvektorn
        write!(f, "Value: {} + {}i, Dimension: {:?}", 
               self.value.re, self.value.im, self.dimension)
    }
}

// Implementera Error trait för QuantityError
impl std::error::Error for QuantityError {}

// Implementera Display för QuantityError
impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantityError::IncompatibleDimensions { message, .. } => {
                write!(f, "Incompatible dimensions: {}", message)
            },
            QuantityError::DimensionOverflow { message, .. } => {
                write!(f, "Dimension overflow: {}", message)
            },
            QuantityError::NumericError { message } => {
                write!(f, "Numeric error: {}", message)
            },
            QuantityError::OrthogonalityError { message, .. } => {
                write!(f, "Orthogonality error: {}", message)
            },
            QuantityError::DivisionByZero => {
                write!(f, "Division by zero")
            },
            QuantityError::ConversionError { message, .. } => {
                write!(f, "Conversion error: {}", message)
            },
        }
    }
}

use std::collections::HashMap;
use once_cell::sync::Lazy;

// Definiera enhetstyper enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quantity {
    Length,
    Time,
    Mass,
    Current,
    Temperature,
    AmountOfSubstance,
    LuminousIntensity,
    Velocity,
    Force,
    Energy,
    Power,
    Torque,
}

// Definiera konstanta dimensionsvektorer
pub const LENGTH: DimensionVector = [1, 0, 0, 0, 0, 0, 0, 0];
pub const TIME: DimensionVector = [0, 1, 0, 0, 0, 0, 0, 0];
pub const MASS: DimensionVector = [0, 0, 1, 0, 0, 0, 0, 0];
include!("dim_const.rs");

// Lazy initialiserad primärtabell
static PRIMETABLE: Lazy<HashMap<DimensionVector, Quantity>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(LENGTH, Quantity::Length);
    map.insert(TIME, Quantity::Time);
    map.insert(MASS, Quantity::Mass);
    include!("primetable_inserts.rs");
    map
});

// Enhetskonstanter med konstruktorer
pub const METER: ValueWithUnit = ValueWithUnit::new(1.0, LENGTH);
pub const SECOND: ValueWithUnit = ValueWithUnit::new(1.0, TIME);
pub const KILOGRAM: ValueWithUnit = ValueWithUnit::new(1.0, MASS);
include!("unit_const.rs");


// Sekundärtabell SI-enheter (exempelvis i en si.rs-fil eller i lib.rs)
static SI_SYMBOLS: Lazy<HashMap<Quantity, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Grundenheter
    map.insert(Quantity::Length, "m");
    map.insert(Quantity::Time, "s");
    map.insert(Quantity::Mass, "kg");
    include!("secondary_inserts.rs");    
    map
});

// Sekundärtabell Anglo-enheter (exempelvis i en anglo.rs-fil)
static ANGLO_SYMBOLS: Lazy<HashMap<Quantity, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Grundenheter
    map.insert(Quantity::Length, "ft");
    map.insert(Quantity::Time, "s");  // Samma som SI
    map.insert(Quantity::Mass, "lb");
    map.insert(Quantity::Current, "A");  // Samma som SI
    map.insert(Quantity::Temperature, "°F");
    map.insert(Quantity::AmountOfSubstance, "mol");  // Samma som SI
    map.insert(Quantity::LuminousIntensity, "cd");  // Samma som SI
    
    // Härledda enheter
    map.insert(Quantity::Velocity, "ft/s");
    map.insert(Quantity::Force, "lbf");
    map.insert(Quantity::Energy, "ft·lbf");
    map.insert(Quantity::Power, "hp");
    map.insert(Quantity::Torque, "ft·lbf");
    
    map
});

// Hjälpfunktioner för att hämta symboler
pub fn get_si_symbol(unit_type: Quantity) -> Option<&'static str> {
    SI_SYMBOLS.get(&unit_type).copied()
}

pub fn get_anglo_symbol(unit_type: Quantity) -> Option<&'static str> {
    ANGLO_SYMBOLS.get(&unit_type).copied()
}

// En generell funktion som kan användas för att hämta symbol baserat på enhetssystem
pub enum UnitSystem {
    SI,
    Anglo,
}

pub fn get_symbol(unit_type: Quantity, system: UnitSystem) -> Option<&'static str> {
    match system {
        UnitSystem::SI => get_si_symbol(unit_type),
        UnitSystem::Anglo => get_anglo_symbol(unit_type),
    }
}