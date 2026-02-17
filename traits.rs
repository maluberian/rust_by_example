
trait Car {
    fn start(&self);
    fn accelerate(&self, speed: i64, direction: String);
    fn stop(&self);
}

struct Ferrari;
impl Car for Ferrari {
    fn start(&self) {
        println!("Ferrari start");
    }

    fn accelerate(&self, speed: i64, direction: String) {
        println!("vrooom vroom vroom going {} towards {}", speed, direction);
    }
    fn stop(&self) {
        println!("screeeeeeeeeee!!!!!");
    }
}

struct Volkswagon;
impl Car for Volkswagon {
    fn start(&self) {
        println!("Nope");
    }
    fn accelerate(&self, speed: i64, direction: String) {
        println!("putt putt putt {} ... cough cough. going nowhere but in the {} direction", speed, direction);
    }

    fn stop(&self) {
        println!("wait... were we even moving");
    }
}

fn get_car(rand_numb: f32) -> Box<dyn Car> {
    if  rand_numb < 0.5 {
        Box::new(Ferrari)
    } else {
        Box::new(Volkswagon)
    }
}

fn main() {
    let car = get_car(0.75);

    car.start();
    car.accelerate(1000, String::from("Ferrari"));
    car.stop();
}