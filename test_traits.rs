struct Person<PetType, PetType2: Animal + Dangerous> where PetType: Animal + NotDangerous {
    first_name: String,
    pet: PetType,
    pet2: PetType2,
}

trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDangerous {
    fn is_dangerous(&self) -> ();
}

trait Dangerous {
    fn is_dangerous(&self) -> ();
}

struct Dog {}
impl NotDangerous for Dog {
    fn is_dangerous(&self) {
        println!("Dog is not Dangerous");
    }
}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog Barked");
    }
}

struct Cat {}
impl NotDangerous for Cat {
    fn is_dangerous(&self) {
        println!("Cat is not dangerous");
    }
}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat Barked");
    }
}

struct Bear {}
impl Dangerous for Bear {
    fn is_dangerous(&self) {
        println!("Bear is dangerous");
    }
}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear Roared");
    }
}

struct Tiger {}
impl Dangerous for Tiger {
    fn is_dangerous(&self) {
        println!("Tiger is dangerous");
    }
}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger Roared");
    }
}

pub fn create_person() {
    let pet1 = Dog{};
    let pet2 = Tiger{};
    let p1 = Person{
        first_name: "Chaitu".to_string(),
        pet: pet1,
        pet2: pet2,
    };

    p1.pet.make_sound();
    p1.pet.is_dangerous();

    p1.pet2.make_sound();
    p1.pet2.is_dangerous();
}
