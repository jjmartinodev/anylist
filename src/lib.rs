use std::any::Any;


trait AnyVec: Any {
    // Little hack because you cannot go from `dyn AnyVec` to `dyn Any`
    // which is needed to call downcast/downcast_mut.
    // This will be possible in the near future when the trait_upcasting
    // feature gets stabilized.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn untyped_pop(&mut self);
    fn untyped_remove(&mut self, index: usize);

    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

impl<T: 'static> AnyVec for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn untyped_pop(&mut self) {
        self.pop();
    }
    fn untyped_remove(&mut self, index: usize) {
        self.remove(index);
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn capacity(&self) -> usize {
        self.capacity()
    }
}

pub struct AnyList(Box<dyn AnyVec>);

impl AnyList {
    pub fn new<T: Any>() -> Self {
        Self(Box::new(Vec::<T>::new()))
    }

    pub fn push<T: Any>(&mut self, value: T) {
        self.0.as_any_mut().downcast_mut::<Vec<T>>().expect("Invalid type").push(value);
    }
    
    pub fn insert<T: Any>(&mut self, index: usize, value: T) {
        self.0.as_any_mut().downcast_mut::<Vec<T>>().expect("Invalid type").insert(index, value);
    }

    pub fn pop<T: Any>(&mut self) -> Option<T> {
        self.0.as_any_mut().downcast_mut::<Vec<T>>().expect("Invalid type").pop()
    }

    pub fn remove<T: Any>(&mut self, index: usize) -> T {
        self.0.as_any_mut().downcast_mut::<Vec<T>>().expect("Invalid type").remove(index)
    }

    pub fn get<T: Any>(&self, index: usize) -> Option<&T> {
        self.0.as_any().downcast_ref::<Vec<T>>().expect("Invalid type").get(index)
    }

    pub fn get_mut<T: Any>(&mut self, index: usize) -> Option<&mut T> {
        self.0.as_any_mut().downcast_mut::<Vec<T>>().expect("Invalid type").get_mut(index)
    }
    
    pub fn untyped_pop(&mut self) {
        self.0.untyped_pop();
    }
    
    pub fn untyped_remove(&mut self, index: usize) {
        self.0.untyped_remove(index);
    }

    pub fn as_slice<T: Any>(&self) -> &[T] {
        self.0.as_any().downcast_ref::<Vec<T>>().expect("Invalid type").as_slice()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    // etc etc
}

#[cfg(test)]
mod tests;