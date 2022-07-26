use crate::mappers::combine::CombineMapper;
use crate::ScalarT;
use std::collections::HashSet;


pub struct DependenciesGetter;


impl CombineMapper for DependenciesGetter {
    type Output = HashSet<String>;
    fn combine(&self, values: &[Self::Output]) -> Self::Output {
        let mut combined_values: Self::Output = HashSet::new();
        for value in values {
            for k in value {
                combined_values.insert(k.to_string());
            }
        }

        return combined_values;
    }
    fn map_variable(&self, name: String) -> Self::Output{
        return HashSet::from([name]);
    }

    fn map_scalar(&self, _value: &ScalarT) -> Self::Output{
        HashSet::new()
    }
}
