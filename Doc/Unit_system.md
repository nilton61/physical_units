# Domänkoncept i ValueWithUnits - Detaljerad sammanfattning

## Problemformulering
Vid fysikaliska beräkningar måste vi kunna hantera olika enhetssystem konsekvent. När en användare påbörjar beräkningar i exempelvis Anglo-systemet (fot, pund, etc.), är det logiskt att resultaten också presenteras i samma system.

## Lösning: UnitSystem
Vi inför konceptet UnitSystem (tidigare benämnt "domän") för att spåra och hantera olika enhetssystem.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum UnitSystem {
    SI,           // Metriska systemet
    Anglo,        // Imperial/US Customary 
    Nautical,     // Nautiska enheter
    Astronomical, // Astronomiska enheter
    Planck,       // Planck-enheter
    CGS,          // Centimeter-gram-sekund
    Natural,      // Naturliga enheter
    Historical,   // Historiska enheter
    Fictional,    // Påhittade enheter
    Custom(u32),  // Användardefinierat system
}
```

## Datastrukturer

### UnitData med UnitSystem
```rust
#[derive(Debug, Clone)]
pub struct UnitData {
    pub dimension: [i8; 8],
    pub symbol: Option<String>,
    pub factor: Option<f64>,
    pub system: UnitSystem,  // Nytt fält för enhetssystem
}
```

### ValueWithUnit med UnitSystem
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,
    pub dimension: [i8; 8],
    pub conversion_factor: f64,
    pub unit_symbol: String,
    pub system: UnitSystem,  // Nytt fält för enhetssystem
}
```

### UnitRegistry med systemindex
```rust
pub struct UnitRegistry {
    // Primär lagring
    units: HashMap<String, UnitData>,
    
    // Index för dimensionssökning
    dimension_index: HashMap<[i8; 8], Vec<String>>,
    
    // Index för symbolsökning
    symbol_index: HashMap<String, String>, // symbol -> unit name
    
    // Nytt index för systemsökning
    system_index: HashMap<(UnitSystem, [i8; 8]), Vec<String>>,
}
```

## Filbaserat enhetssystem
Organisera enheter i separata YAML-filer baserat på UnitSystem:

```
units/
├── si.yaml           # SI-enheter
├── anglo.yaml        # Imperial/US Customary
├── nautical.yaml     # Nautiska enheter 
├── astronomical.yaml # Astronomiska enheter
├── planck.yaml       # Planck-enheter
├── cgs.yaml          # CGS-systemet
├── natural.yaml      # Naturliga enheter
├── historical.yaml   # Historiska enheter
├── fictional.yaml    # Påhittade enheter
└── custom/
    ├── user1.yaml    # Användardefinierad 1
    └── user2.yaml    # Användardefinierad 2
```

## Laddning av enheter från filer
```rust
pub fn load_unit_files(directory: &Path) -> Result<UnitRegistry, LoadError> {
    let mut registry = UnitRegistry::new();
    
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension() == Some(OsStr::new("yaml")) {
            let system = match path.file_stem().unwrap().to_str().unwrap() {
                "si" => UnitSystem::SI,
                "anglo" => UnitSystem::Anglo,
                "nautical" => UnitSystem::Nautical,
                "astronomical" => UnitSystem::Astronomical,
                "planck" => UnitSystem::Planck,
                "cgs" => UnitSystem::CGS,
                "natural" => UnitSystem::Natural,
                "historical" => UnitSystem::Historical,
                "fictional" => UnitSystem::Fictional,
                _ => continue, // Hoppa över okända filer
            };
            
            let content = fs::read_to_string(&path)?;
            let units: Vec<UnitDefinition> = serde_yaml::from_str(&content)?;
            
            for unit in units {
                registry.add_unit(unit, system)?;
            }
        }
    }
    
    Ok(registry)
}
```

## Hantering av Anglo-systemet (US/Imperial)
För enheter som skiljer sig mellan Imperial och US Customary:

```yaml
# I anglo.yaml
- unit: gallon_us
  dimension: [3, 0, 0, 0, 0, 0, 0, 0]  # Volym
  symbol: "gal (US)"
  factor: 0.003785411784

- unit: gallon_imperial
  dimension: [3, 0, 0, 0, 0, 0, 0, 0]  # Volym
  symbol: "gal (UK)"
  factor: 0.00454609
```

För identiska enheter i båda systemen:

```yaml
# I anglo.yaml
- unit: foot
  dimension: [1, 0, 0, 0, 0, 0, 0, 0]  # Längd
  symbol: "ft"
  factor: 0.3048
```

## Enhetssökningslogik
```rust
impl UnitRegistry {
    // Söka enheter med hänsyn till system
    pub fn find_unit_by_dimension_in_system(&self, dim: &[i8; 8], system: UnitSystem) -> Option<&String> {
        // Först söker vi i önskat system
        if let Some(units) = self.system_index.get(&(system, *dim)) {
            return units.first();
        }
        // Fallback till SI
        if system != UnitSystem::SI {
            if let Some(units) = self.system_index.get(&(UnitSystem::SI, *dim)) {
                return units.first();
            }
        }
        // Slutlig fallback till generell sökning
        self.dimension_index.get(dim)?.first()
    }
}
```

## Operationer mellan värden
Vid matematiska operationer mellan värden med olika UnitSystem:

```rust
fn determine_result_system(left: UnitSystem, right: UnitSystem) -> UnitSystem {
    match (left, right) {
        (a, b) if a == b => a,  // Samma system behålls
        (_, UnitSystem::SI) => UnitSystem::SI,  // SI prioriteras vid blandning med SI
        (UnitSystem::SI, _) => UnitSystem::SI,  // SI prioriteras vid blandning med SI
        _ => UnitSystem::SI,    // Fallback till SI vid blandning av andra system
    }
}
```

## Fördelar med denna design
1. **Konsekventa beräkningskedjor** - Enheter från samma system bevaras genom beräkningar
2. **Tydlig organisation** - Filbaserad struktur gör systemet lättare att underhålla
3. **Flexibilitet** - Stöd för många typer av enhetssystem
4. **Pragmatisk hantering** - Anglo-systemet hanterar både Imperial och US-enheter med tydlig markering
5. **Fallback-mekanism** - SI används som standard vid konflikter

Detta skapar ett system som både är kraftfullt för avancerade användare och intuitivt för vardagsanvändning.