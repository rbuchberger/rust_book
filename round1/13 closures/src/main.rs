use closures::cacher::Cacher;
use std::thread;
use std::time::Duration;

fn main() {
    let sim_intensity = 10;
    let sim_random_number = 7;

    generate_workout(sim_intensity, sim_random_number);
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
