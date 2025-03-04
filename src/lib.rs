use num_complex::Complex;
use std::fmt;
use std::ops::Mul;

pub mod units;
use crate::units::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,         // Komplext värde lagrat i SI-enheter
    pub dimension: [i8; 8],          // Dimensionsvektor
    pub conversion_factor: f64,      // Omvandlingsfaktor till användarenhet
    pub unit_symbol: String,         // Symbol för användarenhet i TeX-format
}

// Implementera grundläggande funktionalitet...
