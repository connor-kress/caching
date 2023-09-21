use std::collections::HashMap;

pub struct CachedFunction<Args, Return, Func>
where
    Args: Clone + Eq + std::hash::Hash,
    Return: Clone,
    Func: Fn(Args) -> Return,
{
    f: Func,
    cache: HashMap<Args, Return>,
}

impl<Args, Return, Func> CachedFunction<Args, Return, Func>
where
    Args: Clone + Eq + std::hash::Hash,
    Return: Clone,
    Func: Fn(Args) -> Return,
{
    pub fn new(f: Func) -> Self {
        CachedFunction {
            f,
            cache: HashMap::new(),
        }
    }

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
