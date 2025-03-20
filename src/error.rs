use std::fmt;
use crate::DimensionVector;

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