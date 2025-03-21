// test_tables.rs

use crate::{
    PRIMETABLE, SI_SYMBOLS, Quantity, 
    LENGTH, TIME, MASS, CURRENT, TEMPERATURE, SUBSTANCE, LUMINOUS,
    VELOCITY, FORCE, ENERGY, POWER, TORQUE
};

#[test]
fn test_primary_table_basic_dimensions() {
    // Testa att grundläggande dimensioner mappar till rätt Quantity
    assert_eq!(PRIMETABLE.get(&LENGTH), Some(&Quantity::Length));
    assert_eq!(PRIMETABLE.get(&TIME), Some(&Quantity::Time));
    assert_eq!(PRIMETABLE.get(&MASS), Some(&Quantity::Mass));
    assert_eq!(PRIMETABLE.get(&CURRENT), Some(&Quantity::Current));
    assert_eq!(PRIMETABLE.get(&TEMPERATURE), Some(&Quantity::Temperature));
    assert_eq!(PRIMETABLE.get(&SUBSTANCE), Some(&Quantity::AmountOfSubstance));
    assert_eq!(PRIMETABLE.get(&LUMINOUS), Some(&Quantity::LuminousIntensity));
}

#[test]
fn test_primary_table_derived_dimensions() {
    // Test that derived dimensions map to the correct Quantity
    assert_eq!(PRIMETABLE.get(&VELOCITY), Some(&Quantity::Velocity));
    assert_eq!(PRIMETABLE.get(&FORCE), Some(&Quantity::Force));
    assert_eq!(PRIMETABLE.get(&ENERGY), Some(&Quantity::Energy));
    assert_eq!(PRIMETABLE.get(&POWER), Some(&Quantity::Power));
    assert_eq!(PRIMETABLE.get(&TORQUE), Some(&Quantity::Torque));
}

#[test]
fn test_si_symbols_basic_units() {
    // Testa att grundläggande enheter mappar till korrekt SI-symbol
    assert_eq!(SI_SYMBOLS.get(&Quantity::Length), Some(&"m"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Time), Some(&"s"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Mass), Some(&"kg"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Current), Some(&"A"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Temperature), Some(&"K"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::AmountOfSubstance), Some(&"mol"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::LuminousIntensity), Some(&"cd"));
}

#[test]
fn test_si_symbols_derived_units() {
    // Testa att härledda enheter mappar till korrekt SI-symbol
    assert_eq!(SI_SYMBOLS.get(&Quantity::Velocity), Some(&"m/s"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Force), Some(&"N"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Energy), Some(&"J"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Power), Some(&"W"));
    assert_eq!(SI_SYMBOLS.get(&Quantity::Torque), Some(&"N·m"));
}

#[test]
fn test_dimension_to_symbol_chain() {
    // Testa hela kedjan från dimensionsvektor via Quantity till symbol
    for &dim in &[LENGTH, TIME, MASS, CURRENT, TEMPERATURE, SUBSTANCE, LUMINOUS,
                  VELOCITY, FORCE, ENERGY, POWER, TORQUE] {
        // Steg 1: Dimensionsvektor -> Quantity
        let quantity = PRIMETABLE.get(&dim);
        assert!(quantity.is_some(), "Dimensionsvektor {:?} hittades inte i PRIMETABLE", dim);
        
        // Steg 2: Quantity -> Symbol
        let symbol = SI_SYMBOLS.get(quantity.unwrap());
        assert!(symbol.is_some(), 
                "Quantity {:?} hittades inte i SI_SYMBOLS", quantity.unwrap());
    }
}

#[test]
fn test_table_completeness() {
    // En lista med alla Quantity-värden vi förväntar oss finns
    let expected_quantities = [
        Quantity::Length, Quantity::Time, Quantity::Mass,
        Quantity::Current, Quantity::Temperature, 
        Quantity::AmountOfSubstance, Quantity::LuminousIntensity,
        Quantity::Velocity, Quantity::Force, Quantity::Energy,
        Quantity::Power, Quantity::Torque
    ];
    
    // Kontrollera att varje Quantity finns i SI_SYMBOLS
    for &q in &expected_quantities {
        assert!(SI_SYMBOLS.contains_key(&q), 
                "SI_SYMBOLS saknar Quantity {:?}", q);
    }
    
    // Kontrollera att PRIMETABLE innehåller en dimensionsvektor för varje Quantity
    // Detta är mer komplext eftersom vi behöver söka i PRIMETABLE efter varje Quantity
    for &q in &expected_quantities {
        let found = PRIMETABLE.values().any(|&value| value == q);
        assert!(found, "PRIMETABLE saknar dimensionsvektor för Quantity {:?}", q);
    }
}