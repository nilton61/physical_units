## Reviderad Roadmap för ValueWithUnits v2 Implementation

### Fas 1: Förberedelser och filstruktur

1. **Filorganisering och UnitSystem-baserad struktur**
   - Dela upp den stora YAML-filen i separata filer baserade på UnitSystem
   - Skapa mappstruktur för units/ med si.yaml, anglo.yaml, etc.
   - Definiera lista över standardenheter för varje UnitSystem

2. **Enhetsregister med preferenssystem**
   - Definiera UnitRegistry-struktur med stöd för föredraget system
   - Implementera logik där första laddade YAML-filen bestämmer föredraget system
   - Dokumentera relationerna mellan system för fallback vid enhetsval

3. **Planera teststruktur**
   - Skapa testmönster för filhantering och systeminläsning
   - Definiera tester för preferenssystem och standardenheter
   - Upprätta testmiljö med separata YAML-testfiler

### Fas 2: Grundläggande infrastruktur med Result-inkapslad felhantering

1. **Felhanteringsmekanism**
   - Implementera QuantityError-enum från systembeskrivningen
   - Lägga till UnitSystemError för enhetssystemspecifika fel
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
   - Implementera UnitRegistry-struktur med preferenssystem
   - Implementera UnitSystem-enum med alla nödvändiga varianter
   ```rust
   #[test]
   fn test_unit_registry_creation() {
       let registry = UnitRegistry::new();
       assert_eq!(registry.units.len(), 0);
       assert_eq!(registry.dimension_index.len(), 0);
       assert_eq!(registry.preferred_system, UnitSystem::SI); // Default
   }
   
   #[test]
   fn test_preferred_system_setting() {
       let registry = load_unit_files(&["units/anglo.yaml", "units/si.yaml"]).unwrap();
       assert_eq!(registry.preferred_system, UnitSystem::Anglo);
   }
   ```

3. **YAML-filhantering per UnitSystem med standardenheter**
   - Implementera laddning av enheter från systemspecifika YAML-filer
   - Implementera standardenhetsregister per dimension och UnitSystem
   ```rust
   #[test]
   fn test_loading_si_units() {
       let registry = load_unit_files(&["units/si.yaml"]).unwrap();
       assert!(registry.get_by_name("meter").is_some());
       assert!(registry.get_by_dimension(&[1, 0, 0, 0, 0, 0, 0, 0]).is_some());
       assert_eq!(registry.preferred_system, UnitSystem::SI);
   }
   
   #[test]
   fn test_standard_units() {
       let registry = load_unit_files(&["units/si.yaml", "units/anglo.yaml"]).unwrap();
       let std_length_si = registry.get_standard_unit_for_dimension(
           &[1, 0, 0, 0, 0, 0, 0, 0], UnitSystem::SI).unwrap();
       assert_eq!(std_length_si, "meter");
       
       let std_length_anglo = registry.get_standard_unit_for_dimension(
           &[1, 0, 0, 0, 0, 0, 0, 0], UnitSystem::Anglo).unwrap();
       assert_eq!(std_length_anglo, "foot");
   }
   ```

### Fas 3: ValueWithUnit och grundläggande operationer

1. **ValueWithUnit med UnitSystem och Result-inkapsling**
   - Implementera ValueWithUnit-struktur med UnitSystem-fält
   - Alla operationer returnerar Result<ValueWithUnit, QuantityError>
   - Ta hänsyn till preferenssystem vid enhetsdisplay
   ```rust
   #[test]
   fn test_value_creation() {
       let registry = setup_test_registry();
       let length = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       assert_eq!(length.value.re, 5.0);
       assert_eq!(length.system, UnitSystem::SI);
   }
   
   #[test]
   fn test_value_with_preferred_system() {
       let registry = setup_test_registry_with_anglo_preferred();
       // Värdet skapas med SI-enhet
       let length = ValueWithUnit::new(5.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       // Men när vi visar det med registry kontext, används föredraget system
       assert_eq!(length.display_in_preferred_system(&registry), "16.4042 ft");
   }
   ```

2. **Skalär multiplikation för värdeskapande med systemhantering**
   - Implementera skalär multiplikation (för både f64 och Complex<f64>)
   - Värdeskapande respekterar antingen UnitSystem eller registrets föredraget system
   ```rust
   #[test]
   fn test_scalar_multiplication() {
       let registry = setup_test_registry();
       let unit = registry.get_unit_constant("meter").unwrap();
       let length = 5.0 * unit;
       assert_eq!(length.unwrap().value.re, 5.0);
       assert_eq!(length.unwrap().dimension, [1, 0, 0, 0, 0, 0, 0, 0]);
   }
   
   #[test]
   fn test_scalar_with_preferred_system() {
       let registry = setup_test_registry_with_anglo_preferred();
       // Skapa längd med hjälp av föredraget system
       let length = registry.create_value_in_preferred_system(
           5.0, &[1, 0, 0, 0, 0, 0, 0, 0]).unwrap();
       assert_eq!(length.system, UnitSystem::Anglo);
       assert_eq!(length.unit_symbol, "ft");
   }
   ```

3. **Grundläggande aritmetiska operationer med systemhantering**
   - Implementera addition, subtraktion med dimensionskontroll
   - Implementera logik för resultatets UnitSystem baserat på operandsystem
   ```rust
   #[test]
   fn test_addition_same_dimension_different_systems() {
       let registry = setup_test_registry();
       let meter = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                     "m".to_string(), UnitSystem::SI).unwrap();
       let foot = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 0.3048, 
                                    "ft".to_string(), UnitSystem::Anglo).unwrap();
       
       // Vid addition av olika system behålls icke-SI eller föredraget
       let result = (meter + foot).unwrap();
       assert_eq!(result.system, UnitSystem::Anglo);
       assert_approx_eq!(result.value.re / result.conversion_factor, 
                         1.0 / 0.3048 + 1.0, epsilon = 0.00001);
   }
   ```

### Fas 4: Enhetsregister full implementation med systempreferenser

1. **Komplett enhetsregister med alla index och standardenheter**
   - Implementera dimension_index, symbol_index och system_index
   - Implementera standardenhetsregister för varje dimension och system
   ```rust
   #[test]
   fn test_standard_unit_registry() {
       let registry = setup_test_registry();
       
       // Kontrollera att standardenheter finns för viktiga dimensioner
       let length_si = registry.get_standard_unit_for_dimension(
           &[1, 0, 0, 0, 0, 0, 0, 0], UnitSystem::SI).unwrap();
       assert_eq!(length_si, "meter");
       
       let time_anglo = registry.get_standard_unit_for_dimension(
           &[0, 1, 0, 0, 0, 0, 0, 0], UnitSystem::Anglo).unwrap();
       assert_eq!(time_anglo, "second"); // Tid är samma i båda
       
       let volume_si = registry.get_standard_unit_for_dimension(
           &[3, 0, 0, 0, 0, 0, 0, 0], UnitSystem::SI).unwrap();
       assert_eq!(volume_si, "cubic_meter");
       
       let volume_anglo = registry.get_standard_unit_for_dimension(
           &[3, 0, 0, 0, 0, 0, 0, 0], UnitSystem::Anglo).unwrap();
       assert_eq!(volume_anglo, "cubic_foot");
   }
   ```

2. **Systempreferens och enhetsomvandling**
   - Implementera to_unit, to_system och to_preferred_system funktioner
   ```rust
   #[test]
   fn test_to_preferred_system() {
       let registry = setup_test_registry_with_anglo_preferred();
       let si_length = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                          "m".to_string(), UnitSystem::SI).unwrap();
       
       // Konvertera till föredraget system (Anglo)
       let preferred_length = si_length.to_preferred_system(&registry).unwrap();
       assert_eq!(preferred_length.system, UnitSystem::Anglo);
       assert_eq!(preferred_length.unit_symbol, "ft");
       assert_approx_eq!(preferred_length.value.re / preferred_length.conversion_factor, 
                        3.28084, epsilon = 0.00001);
   }
   ```

3. **Systembestämning för resultat av operationer**
   - Vidareutveckla logik för att bestämma resultatets system
   - Ta hänsyn till preferenssystem vid konflikt
   ```rust
   #[test]
   fn test_system_resolution_with_preferences() {
       let registry = setup_test_registry_with_anglo_preferred();
       
       // Operand från olika system där inget är föredraget system
       let length_si = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                        "m".to_string(), UnitSystem::SI).unwrap();
       let length_nautical = ValueWithUnit::new(1.0, [1, 0, 0, 0, 0, 0, 0, 0], 1852.0, 
                                              "nmi".to_string(), UnitSystem::Nautical).unwrap();
       
       // Vid konflikt används preferenssystem
       let result = (length_si + length_nautical).unwrap();
       assert_eq!(result.system, UnitSystem::Anglo);
   }
   ```

### Fas 5: Formattering, ortogonalitet och avancerade funktioner

1. **SI-prefix formatering med systempreferenser**
   - Implementera Debug- och Display-traits med SI-prefix
   - Ta hänsyn till preferenssystem
   ```rust
   #[test]
   fn test_display_with_system_preference() {
       let registry = setup_test_registry_with_anglo_preferred();
       
       let length = ValueWithUnit::new(1500.0, [1, 0, 0, 0, 0, 0, 0, 0], 1.0, 
                                      "m".to_string(), UnitSystem::SI).unwrap();
       
       // Formatering med SI-prefix
       assert_eq!(format!("{}", length), "1.5 km");
       
       // Formatering med preferenssystem
       assert_eq!(length.display_in_preferred_system(&registry), "4921.26 ft");
   }
   ```

2. **Ortogonalitet och komplex representation**
   - Implementera hantering av ortogonalitet (index 7 i dimensionsvektorn)
   - Implementera komplex representation för fasrelationer
   (Oförändrad från tidigare roadmap)

3. **UnitSystem-baserad standardkonvertering**
   - Implementera automatisk konvertering baserat på standardenheter
   ```rust
   #[test]
   fn test_auto_conversion_to_standard() {
       let registry = setup_test_registry();
       
       // Skapa ett värde i icke-standardenhet
       let small_length = ValueWithUnit::new(10.0, [1, 0, 0, 0, 0, 0, 0, 0], 0.01, 
                                           "cm".to_string(), UnitSystem::SI).unwrap();
       
       // Konvertera till systemets standardenhet
       let std_length = small_length.to_standard_unit(&registry).unwrap();
       assert_eq!(std_length.unit_symbol, "m");
       assert_eq!(std_length.value.re, 0.1);
   }
   ```

### Fas 6: Optimering och avancerad funktionalitet

1. **Prestandaoptimering för systempreferenser**
   - Implementera caching för vanliga operationer
   - Optimera systembaserad enhetssökning
   ```rust
   #[test]
   fn test_performance_system_lookup() {
       let registry = setup_large_test_registry();
       let start = std::time::Instant::now();
       for _ in 0..1000 {
           let _ = registry.get_standard_unit_for_dimension(
               &[1, 0, 0, 0, 0, 0, 0, 0], registry.preferred_system);
       }
       let duration = start.elapsed();
       assert!(duration.as_millis() < 50); // Ska vara snabbt
   }
   ```

2. **Avancerade funktioner med systemnormalisering**
   - Implementera offset-enheter (Celsius, etc.) med systemmedvetenhet
   - Implementera vinkelhantering
   ```rust
   #[test]
   fn test_offset_units_with_system() {
       let registry = setup_test_registry();
       
       // Temperatur i Kelvin (SI)
       let kelvin = ValueWithUnit::new(273.15, [0, 0, 0, 0, 1, 0, 0, 0], 1.0, 
                                      "K".to_string(), UnitSystem::SI).unwrap();
       
       // Konvertera till Celsius (SI)
       let celsius = kelvin.to_unit("°C", &registry).unwrap();
       assert_eq!(celsius.value.re, 0.0);
       assert_eq!(celsius.system, UnitSystem::SI);
       
       // Konvertera till Fahrenheit (Anglo)
       let fahrenheit = kelvin.to_unit("°F", &registry).unwrap();
       assert_eq!(fahrenheit.value.re, 32.0);
       assert_eq!(fahrenheit.system, UnitSystem::Anglo);
   }
   ```

### Fas 7: Fullständig API-dokumentation och användarhandledning

1. **API-dokumentation med systemperspektiv**
   - Dokumentera alla publika funktioner och typer
   - Skapa kodexempel för systemhantering och preferenser
   
2. **Användarhandledning med systemkoncept**
   - Skapa en omfattande guide med exempel på systemhantering
   - Förklara principer för enhetshantering, systemkonvertering, preferenser etc.

## Implementationsordning

För en effektiv utveckling rekommenderas följande ordning:

1. Fas 1, punkt 1-2: Filorganisering och enhetsregister med preferenssystem
2. Fas 2, punkt 1: Felhanteringsmekanism
3. Fas 2, punkt 2-3: Enhetsregister med standardenheter
4. Fas 3, punkt 1-2: ValueWithUnit och skalär multiplikation med systemhantering
5. Fas 3, punkt 3: Grundläggande aritmetiska operationer med systemhantering
6. Fas 4: Komplett enhetsregister med systempreferenser
7. Fas 5-7: Formatering, avancerade funktioner och dokumentation

För varje implementationssteg bör testdriven utveckling tillämpas strikt, där testfallen skrivs först och implementeringen görs för att uppfylla testfallen.