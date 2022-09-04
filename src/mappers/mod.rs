pub mod combine;
pub mod fold;
pub mod identity;
pub mod walk;

pub trait CachedMapper<KT, VT> {
    fn query_cache(&self, key: &KT) -> Option<&VT>;
    fn add_to_cache(&mut self, key: KT, value: VT);
}
