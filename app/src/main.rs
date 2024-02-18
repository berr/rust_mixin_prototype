use derives::{traitify, Mixin};

trait MixinDelegate<T> {
    fn as_inner(&self) -> &T;
    fn as_inner_mut(&mut self) -> &mut T;
}

struct AgeMixin {
    age: u32,
}

#[traitify]
impl AgeMixin {
    pub fn get_age(&self) -> u32 {
        self.age
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age
    }
}

struct NameMixin {
    name: String,
}

#[traitify]
impl NameMixin {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Mixin)]
struct Person {
    age: AgeMixin,
    name: NameMixin,
}

fn work(a: &mut (impl Age + Name)) {
    println!("Hello, {}. Your age is {}", a.get_name(), a.get_age());
    println!("It's your birthday!");
    a.set_age(a.get_age() + 1);
    println!("Your age now is {}", a.get_age());
}

fn main() {
    let mut p = Person {
        age: AgeMixin { age: 10 },
        name: NameMixin {
            name: "Felipe".to_string(),
        },
    };

    work(&mut p);
}
