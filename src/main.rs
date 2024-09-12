use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Clone, PartialEq)]
struct Cat {
    age: usize,
    // static lifetime to not suffer in traits
    name: &'static str,
}

#[derive(Debug)]
struct Dog;

#[derive(Debug)]
enum Pet {
    Cat(Cat),
    Dog(Dog),
}

impl Cat {
    fn name_ptr(&self) -> &&str {
        &self.name
    }
}

impl Debug for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cat {}, age: {}", self.name, self.age)
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

impl From<Pet> for Cat {
    fn from(value: Pet) -> Self {
        match value {
            Pet::Cat(cat) => cat,
            Pet::Dog(_) => {
                panic!("Pet of Dog kind cannot be converted into Cat")
            }
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
            name: "Silvester",
            age: 10,
        };
        cat += 20;
        assert_eq!(cat.age, 30);
    }

    #[test]
    fn test_add_to_cat() {
        let mut cat = Cat {
            name: "Silvester",
            age: 10,
        };
        cat = cat + 20;
        assert_eq!(cat.age, 30);
    }

    #[test]
    #[should_panic]
    fn test_pet_dog_to_cat() {
        let pet = Pet::Dog(Dog);
        let cat: Cat = pet.into();
    }

    #[test]
    fn test_pet_to_cat() {
        let pet = Pet::Cat(Cat {
            name: "Silvester",
            age: 10,
        });
        let cat: Cat = pet.into();
        assert_eq!(
            Cat {
                name: "Silvester",
                age: 10
            },
            cat
        )
    }

    #[test]
    fn test_cat_to_pet() {
        let cat = Cat {
            name: "Silvester",
            age: 10,
        };
        let pet: Pet = cat.into();
        match pet {
            Pet::Cat(cat) => {
                assert_eq!(
                    cat,
                    Cat {
                        name: "Silvester",
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
        name: "catty",
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
    let pet: Pet = cat.into();
    println!("Pet: {pet:?}");

    // Pet to cat
    let cat_from_pet: Cat = pet.into();
    println!("Display cat age after Add  {cat_from_pet}");
}
