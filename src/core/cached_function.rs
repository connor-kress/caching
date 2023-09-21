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

    pub fn call(&mut self, a: Args) -> Return {
        if let Some(res) = self.cache.get(&a) {
            (*res).clone()
        } else {
            let res = (self.f)(a.clone());
            self.cache.insert(a, res.clone());
            res
        }
    }
}