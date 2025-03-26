use crate::{
    // Grundläggande dimensioner
    LENGTH, TIME, MASS, CURRENT, TEMPERATURE, SUBSTANCE, LUMINOUS,
    
    // Härledda dimensioner
    VELOCITY, FORCE, ENERGY, POWER, TORQUE, ACCELERATION, AREA,
    VOLUME, FREQUENCY, PRESSURE, DENSITY, ANGLE, ELECTRIC_POTENTIAL,
    ELECTRIC_RESISTANCE, ELECTRIC_CAPACITANCE, ELECTRIC_CONDUCTANCE,
    INDUCTANCE, MAGNETIC_FLUX, MAGNETIC_FLUX_DENSITY, SPECIFIC_HEAT_CAPACITY,
    HEAT_CAPACITY, ENTROPY, THERMAL_CONDUCTIVITY, DYNAMIC_VISCOSITY,
    KINEMATIC_VISCOSITY, SURFACE_TENSION, RADIANT_INTENSITY, MASS_FLOW,
    MOLAR_MASS, MOLAR_VOLUME, MOLAR_ENERGY, LUMINANCE, ILLUMINANCE,
    SOUND_INTENSITY, ACOUSTIC_IMPEDANCE, WAVE_NUMBER, RADIOACTIVITY,
    ABSORBED_DOSE, CATALYTIC_ACTIVITY, CONCENTRATION, PERMEABILITY,
    PERMITTIVITY, HEAT_TRANSFER_COEFFICIENT, ELASTICITY_MODULUS, JERK,
    SPECIFIC_GAS_VOLUME,
    
    // Övriga beroenden
    PRIMETABLE, SI_SYMBOLS, Quantity, DimensionVector
};

#[test]
fn test_all_dimension_mappings() {
    // Samla alla dimensioner i en array
    let dimensions = [
        // Grundläggande
        LENGTH, TIME, MASS, CURRENT, TEMPERATURE, SUBSTANCE, LUMINOUS,
        
        // Härledda
        VELOCITY, FORCE, ENERGY, POWER, TORQUE, ACCELERATION, AREA,
        VOLUME, FREQUENCY, PRESSURE, DENSITY, ANGLE, ELECTRIC_POTENTIAL,
        ELECTRIC_RESISTANCE, ELECTRIC_CAPACITANCE, ELECTRIC_CONDUCTANCE,
        INDUCTANCE, MAGNETIC_FLUX, MAGNETIC_FLUX_DENSITY, SPECIFIC_HEAT_CAPACITY,
        HEAT_CAPACITY, ENTROPY, THERMAL_CONDUCTIVITY, DYNAMIC_VISCOSITY,
        KINEMATIC_VISCOSITY, SURFACE_TENSION, RADIANT_INTENSITY, MASS_FLOW,
        MOLAR_MASS, MOLAR_VOLUME, MOLAR_ENERGY, LUMINANCE, ILLUMINANCE,
        SOUND_INTENSITY, ACOUSTIC_IMPEDANCE, WAVE_NUMBER, RADIOACTIVITY,
        ABSORBED_DOSE, CATALYTIC_ACTIVITY, CONCENTRATION, PERMEABILITY,
        PERMITTIVITY, HEAT_TRANSFER_COEFFICIENT, ELASTICITY_MODULUS, JERK,
        SPECIFIC_GAS_VOLUME
    ];
    
    // För varje dimension, kontrollera att PRIMETABLE innehåller den
    // och att den mappar till rätt Quantity
    for &dim in &dimensions {
        let quantity = PRIMETABLE.get(&dim);
        assert!(quantity.is_some(), "Dimension {:?} not found in PRIMETABLE", dim);
        
        // Kontrollera att det finns en lämplig symbol i SI_SYMBOLS för quantity
        if let Some(q) = quantity {
            let symbol = SI_SYMBOLS.get(q);
            assert!(symbol.is_some(), "No SI symbol found for {:?}", q);
        }
    }
}

#[test]
fn test_dimension_uniqueness() {
    // Samla alla dimensioner i en vector
    let dimensions = vec![
        // Grundläggande
        LENGTH, TIME, MASS, CURRENT, TEMPERATURE, SUBSTANCE, LUMINOUS,
        
        // Härledda
        VELOCITY, FORCE, ENERGY, POWER, ACCELERATION, AREA,
        VOLUME, FREQUENCY, PRESSURE, DENSITY, ANGLE, ELECTRIC_POTENTIAL,
        ELECTRIC_RESISTANCE, ELECTRIC_CAPACITANCE, ELECTRIC_CONDUCTANCE,
        INDUCTANCE, MAGNETIC_FLUX, MAGNETIC_FLUX_DENSITY, SPECIFIC_HEAT_CAPACITY,
        HEAT_CAPACITY, ENTROPY, THERMAL_CONDUCTIVITY, DYNAMIC_VISCOSITY,
        KINEMATIC_VISCOSITY, SURFACE_TENSION, RADIANT_INTENSITY, MASS_FLOW,
        MOLAR_MASS, MOLAR_VOLUME, MOLAR_ENERGY, LUMINANCE, ILLUMINANCE,
        SOUND_INTENSITY, ACOUSTIC_IMPEDANCE, WAVE_NUMBER, RADIOACTIVITY,
        ABSORBED_DOSE, CATALYTIC_ACTIVITY, CONCENTRATION, PERMEABILITY,
        PERMITTIVITY, HEAT_TRANSFER_COEFFICIENT, ELASTICITY_MODULUS, JERK,
        SPECIFIC_GAS_VOLUME
    ];
    
    // Kontrollera att det inte finns några duplicerade dimensioner
    // (förutom där ortogonalitet skiljer dem åt)
    let mut dim_set = std::collections::HashMap::<[i8; 8], [i8; 8]>::new();
    
    for &dim in &dimensions {
        // Skapa en kopia av dimensionen utan ortogonalitetskomponenten
        let mut base_dim = dim.to_array();
        base_dim[7] = 0; // Ignorera ortogonalitetskomponenten
        
        // Om vi redan har sett denna grundläggande dimension
        if let Some(existing_dim) = dim_set.get(&base_dim) {
            // Vi tillåter duplicering endast om ortogonalitetskomponenten är olika
            assert_ne!(dim[7], existing_dim[7], 
                "Duplicated dimension vector detected (excluding orthogonality): {:?}", base_dim);
        } else {
            dim_set.insert(base_dim, dim.to_array());
        }
    }
}
#[test]
fn test_orthogonality_separation() {
    // Kontrollera specifikt att energi och vridmoment har samma grundläggande 
    // dimensioner men olika ortogonalitetskomponent
    let energy_dim = ENERGY.to_array();
    let torque_dim = TORQUE.to_array();
    
    // Jämför alla komponenter utom ortogonalitet
    for i in 0..7 {
        assert_eq!(energy_dim[i], torque_dim[i]);
    }
    
    // Kontrollera att ortogonalitetskomponenten skiljer dem åt
    assert_ne!(energy_dim[7], torque_dim[7]);
    
    // Verifiera att de mappar till olika Quantity-värden
    let energy_q = PRIMETABLE.get(&ENERGY);
    let torque_q = PRIMETABLE.get(&TORQUE);
    
    assert_eq!(energy_q, Some(&Quantity::Energy));
    assert_eq!(torque_q, Some(&Quantity::Torque));
}

#[test]
fn test_quantity_completeness() {
    // Detta test kontrollerar att varje variant i Quantity-enumen 
    // finns representerad i PRIMETABLE
    
    // Lista över alla Quantity-varianter vi förväntar oss
    let expected_quantities = [
        Quantity::Length, Quantity::Time, Quantity::Mass,
        Quantity::Current, Quantity::Temperature, 
        Quantity::AmountOfSubstance, Quantity::LuminousIntensity,
        Quantity::Velocity, Quantity::Force, Quantity::Energy,
        Quantity::Power, Quantity::Torque, Quantity::Acceleration,
        Quantity::Area, Quantity::Volume, Quantity::Frequency,
        Quantity::Pressure, Quantity::Density, Quantity::Angle,
        Quantity::ElectricPotential, Quantity::ElectricResistance,
        Quantity::ElectricCapacitance, Quantity::ElectricConductance,
        Quantity::Inductance, Quantity::MagneticFlux, Quantity::MagneticFluxDensity,
        Quantity::SpecificHeatCapacity, Quantity::HeatCapacity,
        Quantity::Entropy, Quantity::ThermalConductivity,
        Quantity::DynamicViscosity, Quantity::KinematicViscosity,
        Quantity::SurfaceTension, Quantity::RadiantIntensity,
        Quantity::MassFlow, Quantity::MolarMass, Quantity::MolarVolume,
        Quantity::MolarEnergy, Quantity::Luminance, Quantity::Illuminance,
        Quantity::SoundIntensity, Quantity::AcousticImpedance,
        Quantity::WaveNumber, Quantity::Radioactivity, Quantity::AbsorbedDose,
        Quantity::CatalyticActivity, Quantity::Concentration,
        Quantity::Permeability, Quantity::Permittivity,
        Quantity::HeatTransferCoefficient, Quantity::ElasticityModulus,
        Quantity::Jerk, Quantity::SpecificGasVolume
    ];
    
    // Kontrollera att varje Quantity-variant är inkluderad i PRIMETABLE
    for &q in &expected_quantities {
        let found = PRIMETABLE.values().any(|&value| value == q);
        assert!(found, "PRIMETABLE saknar dimensionsvektor för Quantity {:?}", q);
    }
}