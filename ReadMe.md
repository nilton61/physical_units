# **ValueWithUnits**

A Rust library for robust handling of physical quantities with units, dimensions, and orthogonality.

## **Overview**

ValueWithUnits implements a powerful system for handling physical values with associated units, ensuring dimensional correctness in calculations. The library is designed to be both easy to use and flexible enough for advanced physical calculations.

Key features:

* Complete dimensional analysis with 8-dimensional vector representation  
* Complex value representation for handling phase relationships  
* Orthogonality handling to distinguish physically different quantities with the same dimension  
* Flexible unit management via a database-backed registry system  
* Intuitive API for mathematical operations with automatic unit conversion

## **V2 Unit Registry**

Version 2 introduces a unit registry that separates unit data from code:

* JSON-driven unit management  
* Standard units for each dimension combination  
* Bidirectional lookup (dimension vector → units, symbol → unit)  
* Preservation of user-specified units

## **Installation**

Add the following to your Cargo.toml:

\[dependencies\]  
physical\_units \= "0.2.0"

## **Usage**

// Initialize the unit registry  
physical\_units::initialize("path/to/units.json")?;

// Create values with units  
let length \= 5.0 \* METER;  
let time \= 2.0 \* SECOND;  
let speed \= length / time;  // Result in m/s

// Conversion between units  
let speed\_kph \= speed.to\_unit("km/h");  
println\!("Speed: {}", speed\_kph);  // Displays "Speed: 9 km/h"

// Complex values and orthogonality  
let active\_power \= 100.0 \* WATT;  
let reactive\_power \= 50.0 \* VAR;  
let apparent\_power \= active\_power \+ reactive\_power;  // Complex power

## **Design Principles**

ValueWithUnits is built on the following principles:

1. **Value Integrity**: Values always maintain dimensional integrity and are stored internally in SI units  
2. **User Preference**: User-specified units are preserved when possible through compatible operations  
3. **Complexity Where Needed**: Complex values are used only where meaningful (e.g., AC circuits)  
4. **Data-Driven Configuration**: The unit registry can be customized via external JSON configuration

## **Advanced Concepts**

### **Dimension Vector**

The vector `[i8; 8]` represents exponents for the fundamental dimensions:

1. Length (L) \- meter (m)  
2. Time (T) \- second (s)  
3. Mass (M) \- kilogram (kg)  
4. Electric current (I) \- ampere (A)  
5. Temperature (K) \- kelvin (K)  
6. Amount of substance (N) \- mole (mol)  
7. Luminous intensity (J) \- candela (cd)  
8. Orthogonality (O) \- Represents orthogonality between quantities

### **Orthogonality**

The orthogonality concept allows the library to distinguish between quantities that have the same dimensions but different physical meanings, such as energy (J) and torque (Nm), by combining dimension vector and complex representation.

## **License**

This project is licensed under the GNU General Public License v3 (GPL-3.0).

## **Contributing**

Contributions are welcome\! See CONTRIBUTING.md for details on how to contribute to the project.

