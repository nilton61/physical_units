// Ortogonalitetskonstant
// Används för att skapa ortogonala enheter (t.ex. för att skilja energi från vridmoment)
pub const ORTHOGONAL: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 1],
};

// Dimensionslösa prefix-konstanter

// Större prefix
// Yotta (10^24)
pub const YOTTA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e24, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Zetta (10^21)
pub const ZETTA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e21, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Exa (10^18)
pub const EXA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e18, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Peta (10^15)
pub const PETA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e15, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Tera (10^12)
pub const TERA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e12, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Giga (10^9)
pub const GIGA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e9, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Mega (10^6)
pub const MEGA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e6, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Kilo (10^3)
pub const KILO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e3, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Hekto (10^2)
pub const HECTO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e2, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Deka (10^1)
pub const DECA: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e1, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Mindre prefix
// Deci (10^-1)
pub const DECI: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-1, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Centi (10^-2)
pub const CENTI: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-2, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Milli (10^-3)
pub const MILLI: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-3, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Micro (10^-6)
pub const MICRO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-6, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Nano (10^-9)
pub const NANO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-9, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Pico (10^-12)
pub const PICO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-12, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Femto (10^-15)
pub const FEMTO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-15, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Atto (10^-18)
pub const ATTO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-18, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Zepto (10^-21)
pub const ZEPTO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-21, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};

// Yokto (10^-24)
pub const YOCTO: ValueWithUnit = ValueWithUnit {
    value: Complex::new(1.0e-24, 0.0),
    dimension: [0, 0, 0, 0, 0, 0, 0, 0],
};