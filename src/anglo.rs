// Sekundärtabell Anglo-enheter (exempelvis i en anglo.rs-fil)
pub static ANGLO_SYMBOLS: Lazy<HashMap<Quantity, &'static str>> = Lazy::new(|| {
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