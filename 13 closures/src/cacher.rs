use std::collections::HashMap;
use std::hash;

pub struct Cacher<Calc, Arg, Return>
where
    Arg: Copy + Eq + hash::Hash,
    Calc: Fn(Arg) -> Return,
    Return: Copy,
{
    calculation: Calc,
    values: HashMap<Arg, Return>,
}

impl<Calc, Arg, Return> Cacher<Calc, Arg, Return>
where
    Arg: Copy + Eq + hash::Hash,
    Calc: Fn(Arg) -> Return,
    Return: Copy,
{
    pub fn new(calculation: Calc) -> Cacher<Calc, Arg, Return> {
        Cacher {
            values: HashMap::new(),
            calculation,
        }
    }

    pub fn value(&mut self, arg: Arg) -> Return {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_value() {
        let mut c = Cacher::new(|a| a);
        c.value(1);

        assert_eq!(c.value(1), 1);
    }

    #[test]
    fn caches() {
        let mut c = Cacher::new(|_a| rand::random::<u32>());
        let val = c.value(1);

        assert_eq!(c.value(1), val);
    }

    #[test]
    fn handles_multiple_values() {
        let mut c = Cacher::new(|a| a);
        c.value(1);

        assert_eq!(c.value(2), 2);
    }

    #[test]
    fn handles_multiple_types() {
        let mut c = Cacher::new(|a: &str| a.len());
        c.value("string");

        assert_eq!(c.value("hi"), 2);
    }
}
