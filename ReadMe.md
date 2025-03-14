# ValueWithUnits

A Rust library for robust handling of physical quantities with units, designed with a clear separation of concerns.

## Design Philosophy

ValueWithUnits is built around three core principles:

1. **Separation of concerns**: Clear distinction between creation, calculation, and presentation
2. **Dimensional integrity**: All calculations maintain dimensional correctness
3. **Flexibility**: Support for multiple unit systems and presentation formats

## Core Components

The library is organized into three main functional areas:

### 1. Value Creation

The creation subsystem provides:
- Constants for all standard units and dimensions
- Intuitive syntax for creating values with proper dimensions
- Support for multiple unit systems (SI, Imperial, etc.)
- Easy access to common physical constants

// Examples of value creation
let length = 5.0 * METER;
let time = 10.0 * SECOND;
let mass = 2.5 * KILOGRAM;

### 2. Calculation Engine

The calculation engine focuses on:
- Maintaining dimensional correctness in all operations
- Efficient representation of physical quantities
- Support for complex values and orthogonality
- Comprehensive error handling

// Examples of calculations
let velocity = length / time;
let kinetic_energy = 0.5 * mass * velocity * velocity;

### 3. Presentation System

The presentation system handles:
- Converting values to appropriate units for display
- Formatting with proper precision and unit symbols
- Support for various output formats (plain text, LaTeX, etc.)
- Automatic selection of appropriate unit prefixes

// Examples of presentation
println!("{}", velocity.to_string()); // "0.5 m/s"
println!("{}", velocity.format_with(KMH)); // "1.8 km/h"
println!("{}", kinetic_energy.best_prefix()); // "312.5 mJ"

## Installation

Add this to your `Cargo.toml`:

[dependencies]
physical_units = "0.2.0"

## Basic Usage

use physical_units::prelude::*;

fn main() {
    // Create values
    let distance = 100.0 * METER;
    let time = 9.8 * SECOND;
    
    // Perform calculations
    let velocity = distance / time;
    
    // Present results in different forms
    println!("Velocity: {}", velocity); // SI units with automatic prefix
    println!("Velocity: {}", velocity.format_with(KMH)); // Specific unit
    println!("Velocity: {}", velocity.to_system(IMPERIAL)); // Different system
}

## Advanced Features

- **Orthogonality handling**: Distinguish between physically different quantities with the same dimensions
- **Complex value support**: For quantities with phase relationships
- **Unit registry**: Extensible database of units and conversion factors
- **Custom formatting**: Define your own output formats and styles

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the GNU General Public License v3 (GPL-3.0).