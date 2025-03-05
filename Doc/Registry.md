# Enhetsregister med UnitSystem - Designdokument

## Datastrukturer för enhetsregistret

Detta dokument beskriver designen för ett enhetsregister som stödjer flera enhetssystem, med effektiv sökning och konsekvent enhetshantering.

### Grundläggande typer

```rust
/// Representerar ett enhetssystem identifierat genom en hashad filnamnsidentifierare
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitSystem(pub u64);

/// Data för en enhet i registret
#[derive(Debug, Clone)]
pub struct UnitData {
    pub dimension: [i8; 8],
    pub symbol: Option<String>,
    pub factor: Option<f64>,
    pub system: UnitSystem,
}

/// Värde med associerad enhet och enhetssystem
#[derive(Debug, Clone, PartialEq)]
pub struct ValueWithUnit {
    pub value: Complex<f64>,
    pub dimension: [i8; 8],
    pub conversion_factor: f64,
    pub unit_symbol: String,
    pub system: UnitSystem,
}
```

### Enhetsregistret

```rust
pub struct UnitRegistry {
    // Primär lagring
    units: HashMap<String, UnitData>,
    
    // Index för dimensionssökning
    dimension_index: HashMap<[i8; 8], Vec<String>>,
    
    // Index för symbolsökning
    symbol_index: HashMap<String, String>, // symbol -> unit name
    
    // Index för systemsökning
    system_index: HashMap<(UnitSystem, [i8; 8]), Vec<String>>,
}
```

## Laddning av enheter från filer

Enheterna laddas från separata YAML-filer där filnamnet bestämmer enhetssystemet.

```rust
pub fn load_unit_files(directory: &Path) -> Result<UnitRegistry, LoadError> {
    let mut registry = UnitRegistry::new();
    
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension() == Some(OsStr::new("yaml")) {
            // Hasha filnamnet för att skapa ett UnitSystem
            let file_stem = path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or(LoadError::InvalidFileName)?;
                
            let system = create_unit_system(file_stem);
            
            let content = fs::read_to_string(&path)?;
            let units: Vec<UnitDefinition> = serde_yaml::from_str(&content)?;
            
            for unit in units {
                registry.add_unit(unit.name, unit.data, system)?;
            }
        }
    }
    
    Ok(registry)
}

/// Skapar ett UnitSystem från ett filnamn genom att hasha det
fn create_unit_system(name: &str) -> UnitSystem {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hasher);
    UnitSystem(hasher.finish())
}
```

## Enhetsregisterfunktioner

```rust
impl UnitRegistry {
    /// Skapar ett nytt tomt enhetsregister
    pub fn new() -> Self {
        UnitRegistry {
            units: HashMap::new(),
            dimension_index: HashMap::new(),
            symbol_index: HashMap::new(),
            system_index: HashMap::new(),
        }
    }
    
    /// Lägger till en enhet i registret och uppdaterar alla index
    pub fn add_unit(&mut self, name: String, data: UnitData, system: UnitSystem) -> Result<(), &'static str> {
        // Kontrollera att enhetsnamnet är unikt
        if self.units.contains_key(&name) {
            return Err("Unit name already exists");
        }
        
        let dimension = data.dimension;
        
        // Uppdatera dimensionsindex
        self.dimension_index
            .entry(dimension)
            .or_insert_with(Vec::new)
            .push(name.clone());
        
        // Uppdatera systemindex
        self.system_index
            .entry((system, dimension))
            .or_insert_with(Vec::new)
            .push(name.clone());
        
        // Uppdatera symbolindex om symbol finns
        if let Some(symbol) = &data.symbol {
            // Kontrollera om symbolen redan används
            if self.symbol_index.contains_key(symbol) {
                return Err("Symbol already in use");
            }
            self.symbol_index.insert(symbol.clone(), name.clone());
        }
        
        // Slutligen lagra enheten i primärlagringen
        self.units.insert(name, data);
        Ok(())
    }
    
    /// Hämta enhet från namn
    pub fn get_by_name(&self, name: &str) -> Option<&UnitData> {
        self.units.get(name)
    }
    
    /// Hämta enhetsnamn från dimensionsvektor inom ett specifikt system
    pub fn get_by_dimension_in_system(&self, dimension: &[i8; 8], system: UnitSystem) -> Option<&Vec<String>> {
        // Först söker vi i önskat system
        if let Some(units) = self.system_index.get(&(system, *dimension)) {
            return Some(units);
        }
        // Om inget hittades, använder vi det generella dimensionsindexet
        None
    }
    
    /// Hämta enhetsnamn från dimensionsvektor (alla system)
    pub fn get_by_dimension(&self, dimension: &[i8; 8]) -> Option<&Vec<String>> {
        self.dimension_index.get(dimension)
    }
    
    /// Hämta enhetsnamn från symbol
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&String> {
        self.symbol_index.get(symbol)
    }
    
    /// Hitta bästa enhet för en dimensionsvektor med fallback-kedja
    pub fn find_best_unit(&self, dimension: &[i8; 8], preferred_system: UnitSystem) -> Option<String> {
        // 1. Försök hitta enhet i föredraget system
        if let Some(units) = self.get_by_dimension_in_system(dimension, preferred_system) {
            if !units.is_empty() {
                return Some(units[0].clone());
            }
        }
        
        // 2. Fallback: Hitta SI-enhet (konstant hashvärde för "si")
        let si_system = create_unit_system("si");
        if preferred_system != si_system {
            if let Some(units) = self.get_by_dimension_in_system(dimension, si_system) {
                if !units.is_empty() {
                    return Some(units[0].clone());
                }
            }
        }
        
        // 3. Sista fallback: Använd vilken enhet som helst med rätt dimension
        if let Some(units) = self.get_by_dimension(dimension) {
            if !units.is_empty() {
                return Some(units[0].clone());
            }
        }
        
        None
    }
}
```

## Enhetssystemhantering i beräkningar

När värden med olika enhetssystem kombineras i beräkningar, behöver vi bestämma vilket system som ska användas för resultatet:

```rust
/// Bestämmer vilket enhetssystem som ska användas för resultatet
/// baserat på operandernas system
fn determine_result_system(left: UnitSystem, right: UnitSystem) -> UnitSystem {
    if left == right {
        // Om båda operanderna har samma system, behåll det
        return left;
    }
    
    // Om ett av systemen är SI, prioritera det andra systemet
    let si_system = create_unit_system("si");
    if left == si_system {
        return right;
    }
    if right == si_system {
        return left;
    }
    
    // Om systemen är olika och inget är SI, använd SI som fallback
    si_system
}
```

## Filorganisation

Enhetsregistret laddas från en katalogstruktur som följer denna princip:

```
units/
├── si.yaml           # SI-enheter (enheter från SI-systemet)
├── anglo.yaml        # Anglo-enheter (imperial och US customary)
├── nautical.yaml     # Nautiska enheter (sjömil, knop, etc.)
├── astronomical.yaml # Astronomiska enheter (ljusår, parsec, etc.)
└── custom/
    └── user1.yaml    # Användardefinierade enhetssystem
```

Varje fil innehåller enheter för ett specifikt system, och filnamnet (utan .yaml) blir identifierare för systemet.

## Fördelar med denna design

1. **Effektiv sökning**: Genom att använda hashade systemidentifierare och flera index uppnås effektiv sökning på både enhetsnamn, dimensioner och symboler.

2. **Flexibla enhetssystem**: Nya enhetssystem kan läggas till enkelt genom att skapa nya YAML-filer utan att ändra koden.

3. **Typsäker identifiering**: Genom att använda en dedikerad UnitSystem-typ uppnås bättre typsäkerhet än med rå u64, utan prestandaförlust.

4. **Konsekvent fallback**: När enheter saknas i ett önskat system, används en tydlig prioritetsordning för att välja alternativ.

5. **Effektiv representation**: Genom att använda hashade systemidentifierare minimeras minnesåtgång och jämförelseoperationer blir mycket snabba.