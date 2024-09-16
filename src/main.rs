use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Clone, PartialEq, Debug)]
struct Cat {
    age: usize,
    name: String,
}

#[derive(Debug)]
struct Dog;

#[derive(Debug)]
enum Pet {
    Cat(Cat),
    Dog(Dog),
}

impl Cat {
    fn name_ptr(&self) -> &str {
        &self.name
    }
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cat {}, age: {}", self.name, self.age)
    }
}

impl Add<usize> for Cat {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            age: self.age + other,
            name: self.name,
        }
    }
}

impl From<Cat> for Pet {
    fn from(value: Cat) -> Self {
        Pet::Cat(value)
    }
}

impl TryFrom<Pet> for Cat {
    type Error = &'static str;

    fn try_from(value: Pet) -> Result<Self, Self::Error> {
        match value {
            Pet::Cat(cat) => Ok(cat),
            Pet::Dog(_) => Err("Dog cannot be converted to cat"),
        }
    }
}

impl AddAssign<usize> for Cat {
    fn add_assign(&mut self, other: usize) {
        self.age += other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn test_add_assign_cat() {
        let mut cat = Cat {
            name: String::from("Silvester"),
            age: 10,
        };
        cat += 20;
        assert_eq!(cat.age, 30);
    }

    #[test]
    fn test_add_to_cat() {
        let mut cat = Cat {
            name: String::from("Silvester"),
            age: 10,
        };
        cat = cat + 20;
        assert_eq!(cat.age, 30);
    }

    #[test]
    fn test_pet_dog_to_cat() {
        let pet = Pet::Dog(Dog);
        let cat = Cat::try_from(pet);
        assert_eq!(cat.is_err(), true)
    }

    #[test]
    fn test_pet_to_cat() {
        let pet = Pet::Cat(Cat {
            name: String::from("Silvester"),
            age: 10,
        });
        if let Ok(cat) = Cat::try_from(pet) {
            assert_eq!(
                Cat {
                    name: String::from("Silvester"),
                    age: 10
                },
                cat
            )
        }
    }

    #[test]
    fn test_cat_to_pet() {
        let cat = Cat {
            name: String::from("Silvester"),
            age: 10,
        };
        let pet: Pet = cat.into();
        match pet {
            Pet::Cat(cat) => {
                assert_eq!(
                    cat,
                    Cat {
                        name: String::from("Silvester"),
                        age: 10
                    }
                )
            }
            _ => {}
        }
    }
}

fn main() {
    let mut cat = Cat {
        age: 10,
        name: String::from("catty"),
    };
    println!("Display: {cat}");

    // Show name
    println!("Display cat Name pointer  {:p}", cat.name_ptr());

    // AddAssign
    cat += 5;
    println!("Display cat age after AddAssign  {cat}");

    // AddAssign
    cat = cat + 5;
    println!("Display cat age after Add  {cat}");

    // Cat to Pet
    let mut pet: Pet = cat.into();
    println!("Pet: {pet:?}");

    // Pet to cat
    if let Ok(cat_from_pet) = Cat::try_from(pet) {
        println!("Display cat after extraction from Pet {cat_from_pet}");
    }

    // test try from
    pet = Pet::Dog(Dog);
    if let Err(err) = Cat::try_from(pet) {
        println!("Err: {err}");
    }
}
