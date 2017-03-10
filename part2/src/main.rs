fn main() {
    let mut fred = Person {
        first_name: "Fred".to_string(),
        last_name: "Sanford".to_string(),
        birth_year: 1908,
    };

    fred.set_age(109);
    println!("{}", fred);

    print_name(&fred);

    let fido = Dog {name: "Fido".to_string()};
    print_name(&fido);

    let snoopy = Dog {name: "Snoopy".to_string()};
    let dog;
    {
        let spike = Dog {name: "Spike".to_string()};
        dog = return_first(&snoopy, &spike);
    }

    print_name(dog);
}

fn print_name(nameable: &HasName) {
    println!("{}", nameable.get_name());
}

fn return_first<'a, 'b>(obj1: &'a HasName, obj2: &'b HasName) -> &'a HasName {
    obj1
}

trait HasName {
    fn get_name(&self) -> String;
}

struct Person {
    first_name: String,
    last_name: String,
    birth_year: i16,
}

impl Person {
    fn get_age(&self) -> i16 {
        2017 - self.birth_year
    }

    fn set_age(&mut self, age: i16) {
        self.birth_year = 2017 - age;
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{} {}, age {}",
               self.first_name,
               self.last_name,
               self.get_age())
    }
}

impl HasName for Person {
    fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

struct Dog {
    name: String
}

impl HasName for Dog {
    fn get_name(&self) -> String {
        format!("{}", self.name)
    }
}