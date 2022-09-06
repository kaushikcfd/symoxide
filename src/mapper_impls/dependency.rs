use crate::mappers::combine::CombineMapper;
use crate::mappers::CachedMapper;
use crate::primitives::Expression;
use crate::utils::ExpressionRawPointer;
use crate::ScalarT;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct DependenciesGetter {
    cache: HashMap<ExpressionRawPointer, Rc<HashSet<String>>>,
}

impl CachedMapper<ExpressionRawPointer, Rc<HashSet<String>>> for DependenciesGetter {
    fn query_cache(&self, key: &ExpressionRawPointer) -> Option<&Rc<HashSet<String>>> {
        self.cache.get(&key)
    }
    fn add_to_cache(&mut self, key: ExpressionRawPointer, value: Rc<HashSet<String>>) {
        self.cache.insert(key, value);
    }
}

impl CombineMapper for DependenciesGetter {
    type Output = Rc<HashSet<String>>;

    fn combine(&mut self, values: &[Self::Output]) -> Self::Output {
        let mut combined_values: HashSet<String> = HashSet::new();
        for value in values {
            for k in value.iter() {
                combined_values.insert(k.clone());
            }
        }

        return Rc::new(combined_values);
    }
    fn map_variable(&mut self, name: String) -> Self::Output {
        return Rc::new(HashSet::from([name]));
    }

    fn map_scalar(&mut self, _value: &ScalarT) -> Self::Output {
        Rc::new(HashSet::new())
    }
}

pub fn get_dependencies(expr: &Expression) -> HashSet<String> {
    let mut mapper = DependenciesGetter { cache: HashMap::new() };
    let rc_deps = mapper.visit(&Rc::new(expr.clone()));
    let mut result: HashSet<String> = HashSet::new();
    for k in rc_deps.iter() {
        result.insert(k.clone());
    }

    return result;
}
