# Best-Fit Algoritm och Dubbel Hashning för Dimensionsvektorer

## Bakgrund

I ValueWithUnits-systemet representeras fysikaliska dimensioner genom en 8-dimensionell vektor `[i8; 8]`. Vid presentation av resultat behöver systemet hitta en passande enhetssymbol för en given dimensionsvektor. Om ingen exakt matchning finns i det valda enhetssystemet, behövs en metod för att hitta en "best fit" approximation.

## Dubbel Hashning-strategi

Systemet använder en dubbel hashning-approach för att separera dimensionshantering från presentation:

1. **Primär dimensionshash**: Omvandlar en dimensionsvektor till ett mellanvärde eller en lista med best-fit alternativ när exakt matchning saknas.

2. **Sekundära presentationshashar**: Olika hashar för varje enhetssystem och presentationsstil (SI, Anglo, TeX, Unicode, etc.) som tar dimensionshashens resultat som input och genererar lämpliga enhetsrepresentationer.

Denna separation ger flera fördelar:
- **Flexibilitet**: Samma dimensionsvektor kan representeras olika i olika enhetssystem
- **Moduläritet**: Enhets- och presentationssystem kan uppdateras oberoende av varandra
- **Utbyggbarhet**: Nya enhetssystem kan läggas till utan att påverka grundmotorn
- **Anpassningsbarhet**: Olika presentationsstilar kan tillämpas utan att ändra dimensionshanteringen

- ## Best-Fit Algoritm

När ingen exakt matchning för en dimensionsvektor hittas i ett specifikt enhetssystem, används följande algoritm:

```
Funktion HittaBestFit(dimensionsvektor, enhetssystem):
    1. Sök efter exakt matchning i enhetssystemet
    2. Om matchning hittas, returnera enhetssymbolen
    3. Annars:
       a. Hitta positionen med högsta absolutbeloppet i vektorn
       b. Om flera positioner har samma absolutbelopp, prioritera negativa värden
       c. Minska absolutbeloppet för detta värde med 1 (mot noll)
       d. Sök efter den modifierade vektorn
       e. Om ingen matchning hittas, upprepa från steg 3a
       f. När en matchning hittas, kombinera resultatenheten med grundenheten som 
          motsvarar skillnaden mellan originalvektorn och den matchade vektorn
```

          ### Exempel

Om dimensionsvektorn [2, -3, 1, 0, 0, 0, 0, 0] (t.ex. W) inte finns i ett visst enhetssystem:

1. Identifiera högsta absolutbeloppet: -3 i position 1 (tid)
2. Modifiera till: [2, -2, 1, 0, 0, 0, 0, 0] (t.ex. J/s)
3. Om detta hittas, kombinera med tidsenhet för att återskapa original: J/s²

## Implementationsstruktur

### 1. Dimensionshash

```rust
// Primär hashning// Mappning från enhetssystem till dess presentationsformat
HashMap<UnitSystem, HashMap<String, UnitPresentation>>

struct UnitPresentation {
    plain_text: String,    // Ren text-representation
    tex_format: String,    // TeX-formaterad representation
    unicode_format: String // Unicode-representation när tillgänglig
}
HashMap<[i8; 8], Vec<DimensionUnit>>

struct DimensionUnit {
    unit_id: String,  // Identifierare för enheten
    priority: u8,     // Prioritet (för att välja bland flera alternativ)
}

## Designöverväganden

### Fördelar:

- **Separation av koncept**: Dimensionshantering är skild från presentation
- **Flexibilitet**: Samma dimensionsvektor kan presenteras olika baserat på sammanhang
- **Determinism**: Algoritmen ger konsekvent samma resultat för en given dimensionsvektor
- **Intuition**: Genom att prioritera högsta absolutbelopp och negativa värden skapas ofta fysikaliskt intuitiva enheter

### Viktiga punkter:

1. **Grundenheter**: Alla grundenheter måste finnas representerade i varje enhetssystem för att algoritmen ska fungera korrekt.

2. **Förfyllda tabeller**: Tabellerna bör innehålla många vanliga enheter som standard för att minimera behovet av algoritmen.

3. **Prioritering av negativa värden**: Negativa värden prioriteras vid reduktion för att skapa mer intuitiva bråkuttryck.

4. **Prestanda**: Eftersom algoritmen endast används vid presentation är prestandakraven måttliga.

## Exempel på användning

```rust
// Dimensionsvektor från en beräkning
let dimension = [1, -2, 1, 0, 0, 0, 0, 0]; // N/m²

// Steg 1: Slå upp i dimensionshashen
let unit_options = dimension_hash.get(&dimension)
    .unwrap_or_else(|| best_fit_algorithm(&dimension));

// Steg 2: Välj presentation baserat på enhetssystem
let unit_presentation = presentation_hash
    .get(&unit_system)
    .and_then(|system_hash| system_hash.get(&unit_options[0].unit_id))
    .unwrap_or_default();

// Visa med lämplig formatering
println!("Värde: {}{}", value, unit_presentation.plain_text);

