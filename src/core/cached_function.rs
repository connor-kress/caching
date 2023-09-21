use std::collections::HashMap;

/// A wrapper struct for a function or closure which implements caching.
pub struct CachedFunction<Args, Return, Func>
where
    Args: Clone + Eq + std::hash::Hash,
    Return: Clone,
    Func: Fn(Args) -> Return,
{
    /// The function that the `CachedFunction` instance is wrapping.
    f: Func,
    /// The store of previous calls to the function and their results.
    cache: HashMap<Args, Return>,
}

impl<Args, Return, Func> CachedFunction<Args, Return, Func>
where
    Args: Clone + Eq + std::hash::Hash,
    Return: Clone,
    Func: Fn(Args) -> Return,
{
    /// Creates a new `CachedFunction` instance by providing function.
    /// 
    /// # Arguments
    ///
    /// * `f` - a function which takes and returns singular data types which implement the
    ///         `Clone` trait as well as those required by `std::collections::HashMap`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// fn expensive_function(args: T) -> Bool {
    ///     // expensive operations
    /// }
    /// 
    /// let mut cached_function = CachedFunction::new(expensive_function);
    /// let data = vec![...]  // large
    /// let passed: usize = data.iter().count(|e| cached_function(e));
    /// ...
    /// ```
    pub fn new(f: Func) -> Self {
        CachedFunction {
            f,
            cache: HashMap::new(),
        }
    }

    /// Calls the wrapped function `f` with caching.
    /// 
    /// # Arguments
    /// 
    /// * `args` - The arguments that will be searched in the cache or passed the the underlying function `f`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// ```
    /// let mut cached_function = CachedFunction::new(|args: T| { ... });
    /// 
    /// let args: T = ...
    /// let result = cached_function.call(args);
    /// ```
    pub fn call(&mut self, args: Args) -> Return {
        if let Some(result) = self.cache.get(&args) {
            (*result).clone()
        } else {
            let result = (self.f)(args.clone());
            self.cache.insert(args, result.clone());
            result
        }
    }
}
