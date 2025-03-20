use std::fmt;
use crate::ValueWithUnit;

// Implementera Display för ValueWithUnit
// Detta hanterar endast debugging och default-visning
// Den riktiga presentationen kommer hanteras separat
impl fmt::Display for ValueWithUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Enkel implementation som visar värdet och dimensionsvektorn
        write!(f, "Value: {} + {}i, Dimension: {:?}", 
               self.value.re, self.value.im, self.dimension)
    }
}