use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SharedState<T>(Arc<RwLock<T>>);

impl<T> SharedState<T> {
    pub fn new(state: T) -> Self {
        Self(Arc::new(RwLock::new(state)))
    }

    /// Replaces all instances of this state value with `other` state value,
    /// making them point to the same shared state.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use codasai_server::SharedState;
    /// #
    /// # fn main() {
    /// let state_a0 = SharedState::new(String::from(r#"{ "name": "Juan" }"#));
    /// let state_a1 = state_a0.clone();
    /// let state_a2 = state_a0.clone();
    ///
    /// assert_eq!(state_a0, state_a1);
    /// assert_eq!(state_a1, state_a2);
    ///
    /// let state_b0_val = String::from(r#"{ "name": "Maria" }"#);
    /// state_a2.replace(state_b0_val.clone());
    ///
    /// assert_eq!(state_a2.get(), state_b0_val);
    /// assert_eq!(state_a0.get(), state_b0_val);
    /// assert_eq!(state_a1.get(), state_b0_val);
    ///
    /// # }
    /// ```
    pub fn replace(&self, other: T) {
        let mut inner = self.0.write().unwrap();
        *inner = other;
    }
}

impl<T> SharedState<T>
where T: Clone
{
    /// Returns a clone of the state value
    pub fn get(&self) -> T {
        self.0.read().unwrap().clone()
    }
}

impl<T> PartialEq for SharedState<T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        *self.0.read().unwrap() == *other.0.read().unwrap()
    }
}

impl<T> Eq for SharedState<T> where T: Eq {}
