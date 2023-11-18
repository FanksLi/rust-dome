



pub mod closure {
    use std::time::Duration;    
    use std::thread;
    struct Cacher<T>
    where T: Fn(u32) -> u32, {
        calculation: T,
        value: Option<u32>,
    }
    
    impl<T> Cacher<T>
    where T: Fn(u32) -> u32, {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, num: u32) -> u32 {
    
            match self.value  {
                Some(num) => num,
                None => {
                    let v = (self.calculation)(num);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    // pub fn expensive_closure(num: u32) -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // }
    pub fn generate_workout(intensity: u32, random_number: u32) {
        let mut expensive = Cacher::new(|num| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive.value(intensity));
            println!("Next, do {} situps!", expensive.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive.value(intensity));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::closure::*;
    #[test]
    fn test_generate_workout() {
        let intensity = 10;
        let random_number = 3;
        generate_workout(intensity, random_number);
        assert_eq!(true, false);
    }
}
