use std::fmt::Debug;

pub trait Resource: Debug + Default {
    fn get_name(&self) -> &str;
}

#[derive(Debug)]
pub struct ResourceManager<T: Resource> {
    default: T,
    resources: Vec<T>,
}

impl<T: Resource> ResourceManager<T> {
    pub fn new(resources: Vec<T>, default: T) -> ResourceManager<T> {
        ResourceManager { default, resources }
    }

    pub fn with_default(resources: Vec<T>) -> ResourceManager<T> {
        ResourceManager {
            default: T::default(),
            resources,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn get(&self, id: usize) -> &T {
        self.resources.get(id).unwrap_or(&self.default)
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.resources
            .iter()
            .enumerate()
            .find(|(_i, r)| r.get_name().eq(name))
            .map(|(i, _r)| i)
    }

    pub fn get_names(&self) -> Vec<&str> {
        self.resources.iter().map(|r| r.get_name()).collect()
    }
}
