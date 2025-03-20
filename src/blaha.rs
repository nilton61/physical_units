use num_complex::Complex;
use std::fmt;

// ValueWithUnit struktur
// Representerar ett fysiskt värde med dimensioner, utan presentation
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,   // Komplext värde för att hantera fasrelationer
    pub dimension: [i8; 8],    // Dimensionsvektor [längd, tid, massa, ström, temp, substans, ljus, ortogonalitet]
    // Notera att unit_symbol har tagits bort enligt Const_presentation-designen
}

// QuantityError enum
// Omfattande felhantering för beräkningar med fysikaliska enheter
#[derive(Debug, Clone)]
pub enum QuantityError {
    IncompatibleDimensions {
        left_dim: [i8; 8],
        right_dim: [i8; 8],
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
    pub fn new(value: f64, dimension: [i8; 8]) -> Self {
        ValueWithUnit {
            value: Complex::new(value, 0.0),
            dimension,
        }
    }
    
    // Skapa ett nytt komplext värde med enheter
    pub fn new_complex(re: f64, im: f64, dimension: [i8; 8]) -> Self {
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

// Konstanter för SI-grundenheter
// Dessa kommer att användas för värdeskapande via skalär multiplikation

// Längd - dimension: [1, 0, 0, 0, 0, 0, 0, 0]
pub const METER: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [1, 0, 0, 0, 0, 0, 0, 0],
};

// Tid - dimension: [0, 1, 0, 0, 0, 0, 0, 0]
pub const SECOND: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 1, 0, 0, 0, 0, 0, 0],
};

// Massa - dimension: [0, 0, 1, 0, 0, 0, 0, 0]
pub const KILOGRAM: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 1, 0, 0, 0, 0, 0],
};

// Elektrisk ström - dimension: [0, 0, 0, 1, 0, 0, 0, 0]
pub const AMPERE: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 0, 1, 0, 0, 0, 0],
};

// Temperatur - dimension: [0, 0, 0, 0, 1, 0, 0, 0]
pub const KELVIN: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 0, 0, 1, 0, 0, 0],
};

// Substansmängd - dimension: [0, 0, 0, 0, 0, 1, 0, 0]
pub const MOLE: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 0, 0, 0, 1, 0, 0],
};

// Ljusstyrka - dimension: [0, 0, 0, 0, 0, 0, 1, 0]
pub const CANDELA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 1, 0],
};

