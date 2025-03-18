### Tabellsystem

Vi använder två typer av tabeller för att hantera mappningen mellan dimensioner, storheter och presentationer:

1. **Primärtabell (DIMENSION_TO_QUANTITY)**: Mappar dimensionsvektorer till Quantity-värden
  - Implementerad med `Lazy<HashMap<DimensionVector, Quantity>>` för att undvika dubbeldeklarationer
  - Initieras automatiskt vid första användningen

2. **Sekundärtabeller (SI_SYMBOLS, ANGLO_SYMBOLS)**: Mappar Quantity-värden till enhetssymboler
  - Olika tabeller för olika enhetssystem (SI, Anglo, etc.)
  - Möjliggör konsekvent presentation i olika system

### Konstanthantering

Vi har definierat två typer av konstanter:

1. **Dimensionskonstanter**: Fördefinierade DimensionVector-värden (LENGTH, TIME, MASS, etc.)
2. **Enhetskonstanter**: Fördefinierade ValueWithUnit-instanser (METER, SECOND, KILOGRAM, etc.)

Konstruktorerna för ValueWithUnit är markerade som `const fn` för att möjliggöra användning i konstanta uttryck.

