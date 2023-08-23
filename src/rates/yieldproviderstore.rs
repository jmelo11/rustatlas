use std::rc::Rc;

use super::traits::YieldProvider;

#[derive(Clone)]
pub struct YieldProviderStore {
    providers: Vec<Rc<dyn YieldProvider>>,
    names: Vec<String>,
}

impl YieldProviderStore {
    pub fn new() -> YieldProviderStore {
        YieldProviderStore {
            providers: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, name: String, yield_provider: Rc<dyn YieldProvider>) {
        self.providers.push(yield_provider);
        self.names.push(name);
    }

    pub fn get_provider_by_name(&self, name: String) -> Option<&Rc<dyn YieldProvider>> {
        let item = self.names.iter().enumerate().find_map(
            |(n, s)| {
                if s == &name {
                    Some(n)
                } else {
                    None
                }
            },
        );
        match item {
            Some(id) => self.providers.get(id),
            None => None,
        }
    }

    pub fn get_provider_by_id(&self, id: usize) -> Option<&Rc<dyn YieldProvider>> {
        self.providers.get(id)
    }
}
