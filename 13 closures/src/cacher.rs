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
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_types() {
        let mut c = Cacher::new(|a: &str| a.len());

        c.value("string");
        let v2 = c.value("hi");

        assert_eq!(v2, 2);
    }
}
