use anonymous::implx;

trait Greeter {
    fn greet(&self);
}

fn main() {
    let p = implx! {
        struct { name: String = "Guest".to_string() }
        impl Greeter {
            fn greet(&self) {
                println!("Hello, {}", self.name);
            }
        }
    };
    
    p.greet();

    trait Counter {
        fn increment(&mut self);
        fn get(&self) -> i32;
    }

    let mut c = implx! {
        struct { count: i32 = 0 }
        impl Counter {
            fn increment(&mut self) {
                self.count += 1;
            }
            fn get(&self) -> i32 {
                self.count
            }
        }
    };

    c.increment();
    c.increment();
    println!("Count: {}", c.get());
}
