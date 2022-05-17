use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time::Duration;

fn main() {
    let sim_intensity = 10;
    let sim_random_number = 7;

    generate_workout(sim_intensity, sim_random_number);
}

struct Cacher<Calc, Arg, Return>
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
    fn new(calculation: Calc) -> Cacher<Calc, Arg, Return> {
        Cacher {
            values: HashMap::new(),
            calculation,
        }
    }

    fn value(&mut self, arg: Arg) -> Return {
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

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calc = Cacher::new(|num| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Do {} pushups!", expensive_calc.value(intensity));
        println!("Do {} situps!", expensive_calc.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today.");
        } else {
            println!("Run for {} minutes!", expensive_calc.value(intensity))
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_types() {
    let mut c = Cacher::new(|a: &str| a.len());

    let _v1 = c.value("string");
    let v2 = c.value("hi");

    assert_eq!(v2, 2);
}
