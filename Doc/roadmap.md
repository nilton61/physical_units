## Reviderad Roadmap för ValueWithUnits v2 Implementation

### Fas 1: Dokumentation och planering (nuvarande fas)

1. **Definiera API och enhetsregister**
   - Slutföra dokumentation med enhetsregisterstruktur
   - Definiera filstruktur för YAML-filer uppdelade per UnitSystem
   - Definiera exakt API för ValueWithUnit

2. **Planera teststruktur**
   - Skapa testmönster för varje huvudfunktion
   - Definiera testfall för gränsfall och felhantering
   - Upprätta testmiljö och struktur

### Fas 2: Grundläggande infrastruktur med Result-inkapslad felhantering

1. **Felhanteringsmekanism**
   - Implementera QuantityError-enum från systembeskrivningen
   ```rust
   #[test]
   fn test_error_creation() {
       let error = QuantityError::IncompatibleDimensions {
           left_dim: [1, 0, 0, 0, 0, 0, 0, 0],
           right_dim: [0, 1, 0, 0, 0, 0, 0, 0],
           message: "Cannot add meter and second".to_string()
       };
       assert!(matches!(error, QuantityError::IncompatibleDimensions { .. }));
   }
   ```

2. **Enhetsregister grundläggande**
   - Implementera UnitRegistry-struktur med grundläggande funktioner
   - Implementera UnitSystem-enum med alla nödvändiga varianter
   ```rust
   #[test]
   fn test_unit_registry_creation() {
       let registry = UnitRegistry::new();
       assert_eq!(registry.units.len(), 0);
       assert_eq!(registry.dimension_index.len(), 0);
   }
   ```

3. **YAML-filhantering per UnitSystem**
   - Implementera laddning av enheter från systemspecifika YAML-filer
   ```rust
   #[test]
   fn test_loading_si_units() {
       let registry = load_unit_files("units/si.yaml").unwrap();
       assert!(registry.get_by_name("meter").is_some());
       assert!(registry.get_by_dimension(&[1, 0, 0, 0, 0, 0, 0, 0]).is_some());
   }
   
   #[test]
   fn test_loading_anglo_units() {
       let registry = load_unit_files("units/anglo.yaml").unwrap();
       assert!(registry.get_by_name("foot").is_some());
       assert_eq!(registry.get_by_name("foot").unwrap().system, UnitSystem::Anglo);
   }
   ```

### Fas 3: ValueWithUnit och grundläggande operationer

1. **ValueWithUnit med UnitSystem och Result-inkapsling**
   - Implementera ValueWithUnit-struktur med UnitSystem-fält
   - Alla operationer returnerar Result<ValueWithUnit, QuantityError>
   ```rust
   #[test]
   fn test_value_creation() {
       let registry = setup_test_registry();
       let length = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       assert_eq!(length.value.re, 5.0);
       assert_eq!(length.system, UnitSystem::SI);
   }
   ```

2. **Skalär multiplikation för värdeskapande**
   - Implementera skalär multiplikation (för både f64 och Complex<f64>)
   - Denna är kritisk för värdeskapande enligt design
   ```rust
   #[test]
   fn test_scalar_multiplication() {
       let registry = setup_test_registry();
       let unit = registry.get_unit_constant("meter").unwrap();
       let length = 5.0 * unit;
       assert_eq!(length.unwrap().value.re, 5.0);
       assert_eq!(length.unwrap().dimension, [1, 0, 0, 0, 0, 0, 0, 0]);
   }
   ```

3. **Grundläggande aritmetiska operationer med felhantering**
   - Implementera addition, subtraktion med dimensionskontroll
   - Implementera multiplikation och division med dimensionsberäkning
   ```rust
   #[test]
   fn test_addition_same_dimension() {
       let registry = setup_test_registry();
       let length1 = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                       "m".to_string(), UnitSystem::SI).unwrap();
       let length2 = ValueWithUnit::new(3.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                       "m".to_string(), UnitSystem::SI).unwrap();
       let result = (length1 + length2).unwrap();
       assert_eq!(result.value.re, 8.0);
       assert_eq!(result.dimension, [1, 0, 0, 0, 0, 0, 0, 0]);
   }
   
   #[test]
   fn test_addition_incompatible_dimensions() {
       let registry = setup_test_registry();
       let length = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       let time = ValueWithUnit::new(3.0, [0, 1, 0, 0, 0, 0, 0, 0], 1.0, 
                                    "s".to_string(), UnitSystem::SI).unwrap();
       let result = length + time;
       assert!(result.is_err());
       assert!(matches!(result.unwrap_err(), 
                        QuantityError::IncompatibleDimensions { .. }));
   }
   
   #[test]
   fn test_multiplication() {
       let registry = setup_test_registry();
       let length = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       let time = ValueWithUnit::new(2.0, [0, 1, 0, 0, 0, 0, 0, 0], 1.0, 
                                    "s".to_string(), UnitSystem::SI).unwrap();
       let result = (length * time).unwrap();
       assert_eq!(result.value.re, 10.0);
       assert_eq!(result.dimension, [1, 1, 0, 0, 0, 0, 0, 0]);
   }
   ```

### Fas 4: Enhetsregister full implementation

1. **Komplett enhetsregister med alla index**
   - Implementera dimension_index, symbol_index och system_index
   - Implementera UnitSystem-specifika sökfunktioner
   ```rust
   #[test]
   fn test_dimension_lookup() {
       let registry = setup_test_registry();
       let units = registry.get_by_dimension(&[1, 0, 0, 0, 0, 0, 0, 0]).unwrap();
       assert!(units.contains(&"meter".to_string()));
       assert!(units.contains(&"foot".to_string()));
   }
   
   #[test]
   fn test_system_specific_lookup() {
       let registry = setup_test_registry();
       let units = registry.get_by_dimension_in_system(
           &[1, 0, 0, 0, 0, 0, 0, 0], UnitSystem::Anglo).unwrap();
       assert!(units.contains(&"foot".to_string()));
       assert!(!units.contains(&"meter".to_string()));
   }
   ```

2. **Enhetsomvandling mellan system**
   - Implementera to_unit och to_system funktioner
   ```rust
   #[test]
   fn test_unit_conversion() {
       let registry = setup_test_registry();
       let meters = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                       "m".to_string(), UnitSystem::SI).unwrap();
       let feet = meters.to_unit("foot", &registry).unwrap();
       assert_eq!(feet.system, UnitSystem::Anglo);
       assert_approx_eq!(feet.value.re, 3.28084, epsilon = 0.00001);
   }
   
   #[test]
   fn test_system_conversion() {
       let registry = setup_test_registry();
       let si_length = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                          "m".to_string(), UnitSystem::SI).unwrap();
       let anglo_length = si_length.to_system(UnitSystem::Anglo, &registry).unwrap();
       assert_eq!(anglo_length.system, UnitSystem::Anglo);
       assert_eq!(anglo_length.unit_symbol, "ft");
   }
   ```

3. **Systembestämning för resultat av operationer**
   - Implementera logik för att bestämma vilket system resultatet ska tillhöra
   ```rust
   #[test]
   fn test_system_determination_for_operations() {
       let registry = setup_test_registry();
       let si_length = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                         "m".to_string(), UnitSystem::SI).unwrap();
       let anglo_length = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 0.3048, 
                                            "ft".to_string(), UnitSystem::Anglo).unwrap();
       
       // När olika system används, prioriteras icke-SI
       let result = (si_length + anglo_length).unwrap();
       assert_eq!(result.system, UnitSystem::Anglo);
       
       // När samma system används, behålls det
       let result2 = (anglo_length * anglo_length).unwrap();
       assert_eq!(result2.system, UnitSystem::Anglo);
   }
   ```

### Fas 5: Formattering och ortogonalitet

1. **SI-prefix formatering**
   - Implementera Debug- och Display-traits med SI-prefix
   - Visa 1-3 signifikanta siffror till vänster om decimaltecknet
   ```rust
   #[test]
   fn test_display_with_prefix() {
       let tiny = ValueWithUnit::new(0.000001, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                    "m".to_string(), UnitSystem::SI).unwrap();
       assert_eq!(format!("{}", tiny), "1 µm");
       
       let huge = ValueWithUnit::new(1500000.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                    "m".to_string(), UnitSystem::SI).unwrap();
       assert_eq!(format!("{}", huge), "1.5 Mm");
   }
   ```

2. **Ortogonalitet och komplex representation**
   - Implementera hantering av ortogonalitet (index 7 i dimensionsvektorn)
   - Implementera komplex representation för fasrelationer
   ```rust
   #[test]
   fn test_orthogonality() {
       let registry = setup_test_registry();
       let energy = ValueWithUnit::new(10.0, [2, -2, 1, 0, 0, 0, 0, 0], 1.0, 
                                      "J".to_string(), UnitSystem::SI).unwrap();
       let torque = ValueWithUnit::new(10.0, [2, -2, 1, 0, 0, 0, 0, 1], 1.0, 
                                      "Nm".to_string(), UnitSystem::SI).unwrap();
       
       // Addition ska misslyckas trots samma grunddimension, pga ortogonalitet
       let result = energy + torque;
       assert!(result.is_err());
       assert!(matches!(result.unwrap_err(), 
                        QuantityError::OrthogonalityError { .. }));
   }
   
   #[test]
   fn test_complex_representation() {
       let registry = setup_test_registry();
       let active = ValueWithUnit::new_complex(100.0, 0.0, [2, -3, 1, 0, 0, 0, 0, 0], 
                                             1.0, "W".to_string(), UnitSystem::SI).unwrap();
       let reactive = ValueWithUnit::new_complex(0.0, 50.0, [2, -3, 1, 0, 0, 0, 0, 0], 
                                               1.0, "VAr".to_string(), UnitSystem::SI).unwrap();
       
       // Addition ska lyckas för komplex effekt
       let apparent = (active + reactive).unwrap();
       assert_eq!(apparent.value.re, 100.0);
       assert_eq!(apparent.value.im, 50.0);
       assert_eq!(apparent.dimension, [2, -3, 1, 0, 0, 0, 0, 0]);
   }
   ```

### Fas 6: Optimering och avancerad funktionalitet

1. **Prestandaoptimering**
   - Implementera caching för vanliga operationer
   - Optimera lagringsstrukturer
   ```rust
   #[test]
   fn test_performance_dimension_lookup() {
       let registry = setup_large_test_registry();
       let start = std::time::Instant::now();
       for _ in 0..1000 {
           let _ = registry.get_by_dimension(&[1, 0, 0, 0, 0, 0, 0, 0]);
       }
       let duration = start.elapsed();
       assert!(duration.as_millis() < 50); // Ska vara snabbt
   }
   ```

2. **Avancerade funktioner**
   - Implementera offset-enheter (Celsius, etc.)
   - Implementera vinkelhantering
   ```rust
   #[test]
   fn test_offset_units() {
       let registry = setup_test_registry();
       let kelvin = ValueWithUnit::new(273.15, [0, 0, 0, 0, 1, 0, 0, 0], 1.0, 
                                      "K".to_string(), UnitSystem::SI).unwrap();
       let celsius = kelvin.to_unit("°C", &registry).unwrap();
       assert_eq!(celsius.value.re, 0.0);
   }
   
   #[test]
   fn test_angle_conversion() {
       let registry = setup_test_registry();
       let radians = ValueWithUnit::new(std::f64::consts::PI, [0, 0, 0, 0, 0, 0, 0, 0], 
                                       1.0, "rad".to_string(), UnitSystem::SI).unwrap();
       let degrees = radians.to_unit("°", &registry).unwrap();
       assert_eq!(degrees.value.re, 180.0);
   }
   ```

### Fas 7: Fullständig API-dokumentation och användarhandledning

1. **API-dokumentation**
   - Dokumentera alla publika funktioner och typer
   - Skapa kodexempel för vanliga användningsfall
   
2. **Användarhandledning**
   - Skapa en omfattande guide med exempel
   - Förklara principer för enhetshantering, systemkonvertering, etc.

## Implementationsordning

För en effektiv utveckling rekommenderas följande ordning:

1. Fas 2, punkt 1: Felhanteringsmekanism (kritisk för alla operationer)
2. Fas 3, punkt 1-2: ValueWithUnit och skalär multiplikation (kritisk för skapande)
3. Fas 2, punkt 2-3: Enhetsregister och YAML-filhantering
4. Fas 3, punkt 3: Grundläggande aritmetiska operationer
5. Fas 4: Enhetsregister full implementation
6. Fas 5: Formattering och ortogonalitet
7. Fas 6-7: Optimering, avancerad funktionalitet och dokumentation

För varje implementationssteg bör testdriven utveckling tillämpas strikt, där testfallen skrivs först och implementeringen görs för att uppfylla testfallen.