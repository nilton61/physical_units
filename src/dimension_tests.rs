use crate::{
    DimensionVector, QuantityError,
};
mod dimension_tests {
    use super::*;

    #[test]
    fn test_dimension_addition() {
        // Grundläggande addition
        let dim1 = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);  // LENGTH
        let dim2 = DimensionVector([0, 1, 0, 0, 0, 0, 0, 0]);  // TIME
        let result = (dim1 + dim2).unwrap();
        assert_eq!(result, DimensionVector([1, 1, 0, 0, 0, 0, 0, 0]));
        
        // Addition av samma dimension
        let dim3 = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);  // LENGTH
        let dim4 = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);  // LENGTH
        let result = (dim3 + dim4).unwrap();
        assert_eq!(result, DimensionVector([2, 0, 0, 0, 0, 0, 0, 0]));
    }  // test_dimension_addition

    #[test]
    fn test_dimension_subtraction() {
        // Grundläggande subtraktion
        let dim1 = DimensionVector([1, 1, 0, 0, 0, 0, 0, 0]);  // LENGTH, TIME
        let dim2 = DimensionVector([0, 1, 0, 0, 0, 0, 0, 0]);  // TIME
        let result = (dim1 - dim2).unwrap();
        assert_eq!(result, DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]));  // LENGTH
        
        // Subtraktion till negativ dimension
        let dim3 = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);  // LENGTH
        let dim4 = DimensionVector([2, 0, 0, 0, 0, 0, 0, 0]);  // LENGTH^2
        let result = (dim3 - dim4).unwrap();
        assert_eq!(result, DimensionVector([-1, 0, 0, 0, 0, 0, 0, 0]));  // LENGTH^-1
    }  // test_dimension_subtraction

    #[test]
    fn test_dimension_overflow() {
        // Skapa en dimensionsvektor på gränsen till overflow
        let dim1 = DimensionVector([127, 0, 0, 0, 0, 0, 0, 0]);
        let dim2 = DimensionVector([1, 0, 0, 0, 0, 0, 0, 0]);
        
        // Detta bör resultera i ett overflow-fel
        let result = dim1 + dim2;
        assert!(result.is_err());  // Kontrollera att det blev ett fel
        
        if let Err(QuantityError::DimensionOverflow { dimension_index, attempted_value, .. }) = result {
            assert_eq!(dimension_index, 0);  // Kontrollera rätt dimensionsindex
            assert_eq!(attempted_value, 128);  // Kontrollera rätt värde
        } else {
            panic!("Expected DimensionOverflow error");  // Feltypen matchar inte
        }
    }  // test_dimension_overflow
    
    #[test]
    fn test_deref_functionality() {
        // Testa att Deref fungerar för indexering
        let dim = DimensionVector([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(dim[0], 1);
        assert_eq!(dim[3], 4);
        
        // Testa att iterering fungerar via Deref
        let sum: i8 = dim.iter().sum();
        assert_eq!(sum, 36);
    }  // test_deref_functionality
    
    #[test]
    fn test_from_conversion() {
        // Testa konvertering från array till DimensionVector
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        let dim = DimensionVector::from(array);
        assert_eq!(dim, DimensionVector([1, 2, 3, 4, 5, 6, 7, 8]));
        
        // Testa .into() syntax
        let dim2: DimensionVector = array.into();
        assert_eq!(dim2, DimensionVector([1, 2, 3, 4, 5, 6, 7, 8]));
    }  // test_from_conversion
    
    #[test]
    fn test_to_array() {
        // Testa konvertering från DimensionVector tillbaka till array
        let dim = DimensionVector([1, 2, 3, 4, 5, 6, 7, 8]);
        let array = dim.to_array();
        assert_eq!(array, [1, 2, 3, 4, 5, 6, 7, 8]);
    }  // test_to_array
}  // mod dimension_tests