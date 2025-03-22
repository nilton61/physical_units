use num_complex::Complex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub mod error;
pub use error::QuantityError;

pub mod display;
pub use display::*;

pub mod anglo;
pub use anglo::ANGLO_SYMBOLS;



// Ändra från typalias till en struct med inre array
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionVector(pub [i8; 8]);

// Implementera Deref för att behålla array-liknande beteende
use std::ops::{Deref, DerefMut};

impl Deref for DimensionVector {
    type Target = [i8; 8];
    
    fn deref(&self) -> &Self::Target {
        &self.0  // Returnera referens till den inre arrayen
    } //fn deref
} //impl Deref

impl DerefMut for DimensionVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0  // Returnera muterbar referens till den inre arrayen
    } //fn deref_mut
} //impl DerefMut

// För att tillåta enkel skapande från array
impl From<[i8; 8]> for DimensionVector {
    fn from(array: [i8; 8]) -> Self {
        DimensionVector(array)  // Omslut arrayen i DimensionVector-strukturen
    } //fn from
} //impl From

// Metoder för DimensionVector
impl DimensionVector {
    // Konvertera tillbaka till en vanlig array
    pub fn to_array(self) -> [i8; 8] {
        self.0  // Returnera den inre arrayen
    } //fn to_array
} //impl DimensionVector

// Implementera Add för DimensionVector
impl std::ops::Add for DimensionVector {
    type Output = Result<DimensionVector, QuantityError>;

    fn add(self, other: Self) -> Self::Output {
        let mut result = [0; 8];  // Resultatarray för summan
        
        for i in 0..8 {  // Iterera genom varje dimensionskomponent
            // Beräkna summan som i16 för att fånga potentiella överflöden
            let sum = self[i] as i16 + other[i] as i16;
           
            // Kontrollera om summan överstiger i8:s gränser
            if sum > i8::MAX as i16 || sum < i8::MIN as i16 {  // Overflow-kontroll
                return Err(QuantityError::DimensionOverflow {
                    dimension_index: i,
                    attempted_value: sum,
                    message: format!("Dimension overflow: exponent {} for index {} exceeds limits", sum, i)
                });  // Returnera fel vid overflow
            }  // if overflow-kontroll
           
            result[i] = sum as i8;  // Tilldela summan till resultatarrayen
        }  // for varje dimensionskomponent
        
        Ok(DimensionVector(result))  // Returnera framgångsresultat
    }  // fn add
}  // impl Add

// Implementera Sub för DimensionVector
impl std::ops::Sub for DimensionVector {
    type Output = Result<DimensionVector, QuantityError>;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = [0; 8];  // Resultatarray för differensen
        
        for i in 0..8 {  // Iterera genom varje dimensionskomponent
            // Beräkna differensen som i16 för att fånga potentiella överflöden
            let diff = self[i] as i16 - other[i] as i16;
           
            // Kontrollera om differensen överstiger i8:s gränser
            if diff > i8::MAX as i16 || diff < i8::MIN as i16 {  // Overflow-kontroll
                return Err(QuantityError::DimensionOverflow {
                    dimension_index: i,
                    attempted_value: diff,
                    message: format!("Dimension overflow: exponent {} for index {} exceeds limits", diff, i)
                });  // Returnera fel vid overflow
            }  // if overflow-kontroll
           
            result[i] = diff as i8;  // Tilldela differensen till resultatarrayen
        }  // for varje dimensionskomponent
        
        Ok(DimensionVector(result))  // Returnera framgångsresultat
    }  // fn sub
}  // impl Sub

// ValueWithUnit struktur
// Representerar ett fysiskt värde med dimensioner, utan presentation
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,   // Komplext värde för att hantera fasrelationer
    pub dimension: DimensionVector,    // Dimensionsvektor [längd, tid, massa, ström, temp, substans, ljus, ortogonalitet]
    // Notera att unit_symbol har tagits bort enligt Const_presentation-designen
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
pub const LENGTH: DimensionVector = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);
pub const TIME: DimensionVector = DimensionVector([0, 1, 0, 0, 0, 0, 0, 0]);
pub const MASS: DimensionVector = DimensionVector([0, 0, 1, 0, 0, 0, 0, 0]);
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

#[cfg(test)] mod test_tables;
#[cfg(test)] mod dimension_tests;