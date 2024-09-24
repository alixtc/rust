mod descending_order;
mod kyu_7;
mod kyu_6;

use std::fmt;

enum Status {
    Alive,
    Dead,
}

enum DrivingStatus {
    Running,
    Stopped,
}

struct Person {
    name: String,
    age: u32,
    status: Status,
}

struct Car<'a> {
    owner: &'a Person,
    driving_status: DrivingStatus,
    remaining_gas: u32,
}

impl Car<'_> {
    fn driving_like_a_madman(&mut self) -> &mut Self {
        if let DrivingStatus::Stopped = self.driving_status {
            return self;
        } else {
            while self.remaining_gas > 0 {
                self.remaining_gas -= 1;
                println!(
                    "{} keeps driving with {} remaining gas liters in the tank",
                    self.owner.name,
                    self.remaining_gas
                );
            }
        }
        self.driving_status = DrivingStatus::Stopped;
        println!(
            "{} is dumb, {} has stalled the engine",
            self.owner.name, self.owner.name
        );
        self
    }

    fn refill_gas_tank(&mut self, liters: u32) -> &mut Self {
        self.remaining_gas += liters;
        self.driving_status = DrivingStatus::Running;
        self
    }
}

impl Person {
    pub fn new(name: String, age: u32) -> Person {
        Person {
            name,
            age,
            status: Status::Alive,
        }
    }

    fn aging(&mut self) -> &mut Self {
        self.age += 1;
        self
    }

    fn died(&mut self) {
        self.age += 1;
        self.status = Status::Dead;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Status::Alive => write!(f, "This is {}, he is {}", self.name, self.age),
            Status::Dead => write!(f, "This is {}, he was {} when he died", self.name, self.age),
        }
    }
}

fn main() {
    let mut bob = Person::new("Bob".to_string(), 35);
    println!("{}", bob);
    bob.aging();
    println!("{}", bob);
    let mut car = Car {
        owner: &bob,
        driving_status: DrivingStatus::Running,
        remaining_gas: 10,
    };
    car.driving_like_a_madman()
        .refill_gas_tank(5)
        .driving_like_a_madman();
    bob.aging().died();
    println!("{}", bob);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aging_should_increase_person_age() {
        let mut bob = Person::new("Bob".to_string(), 35);
        bob.aging().aging();
        assert_eq!(bob.age, 37);
    }

    #[test]
    fn driving_like_a_madman_should_empty_gas_tank() {
        let bob = Person::new("Bob".to_string(), 35);
        let mut toyota = Car {
            owner: &bob,
            driving_status: DrivingStatus::Running,
            remaining_gas: 5,
        };
        toyota.driving_like_a_madman();
        assert_eq!(toyota.remaining_gas, 0);
    }

    #[test]
    fn refill_gas_tank_allows_to_drive_car_again() {
        let bob = Person::new("Bob".to_string(), 35);
        let mut toyota = Car {
            owner: &bob,
            driving_status: DrivingStatus::Stopped,
            remaining_gas: 0,
        };
        toyota.refill_gas_tank(5);
        assert_eq!(toyota.remaining_gas, 5);
        toyota.driving_like_a_madman();
        assert_eq!(toyota.remaining_gas, 0);
    }

    #[test]
    fn car_can_alternate_between_driving_and_refill() {
        let bob = Person::new("Bob".to_string(), 35);
        let mut toyota = Car {
            owner: &bob,
            driving_status: DrivingStatus::Stopped,
            remaining_gas: 0,
        };
        toyota
            .refill_gas_tank(5)
            .driving_like_a_madman()
            .refill_gas_tank(5)
            .driving_like_a_madman();
        assert_eq!(toyota.remaining_gas, 0);
    }
}
