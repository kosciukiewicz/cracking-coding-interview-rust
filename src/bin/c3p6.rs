use std::fmt::{Debug, Formatter};

#[derive(Debug, PartialEq)]
enum AnimalType {
    Dog,
    Cat,
}

trait Animal {
    fn get_id(&self) -> i32;
    fn get_type(&self) -> AnimalType;
    fn new(id: i32) -> Self
    where
        Self: Sized;
}

struct Cat {
    id: i32,
}

impl Animal for Cat {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_type(&self) -> AnimalType {
        AnimalType::Cat
    }

    fn new(id: i32) -> Self {
        Cat { id }
    }
}

struct Dog {
    id: i32,
}

impl Animal for Dog {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_type(&self) -> AnimalType {
        AnimalType::Dog
    }

    fn new(id: i32) -> Self {
        Dog { id }
    }
}

struct AnimalToAdopt {
    queued_timestamp: i32,
    animal: Box<dyn Animal>,
}

impl Debug for AnimalToAdopt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "AnimalToAdopt: timestamp={:?}, id={:?}, type={:?}",
            self.queued_timestamp,
            self.animal.get_id(),
            self.animal.get_type()
        ))
    }
}

#[derive(Debug, Default)]
struct AnimalShelter {
    dogs: Vec<AnimalToAdopt>,
    cats: Vec<AnimalToAdopt>,
}

impl AnimalShelter {
    fn enqueue(&mut self, animal: Box<dyn Animal>) {
        let number_of_animals: i32 = (self.cats.len() + self.dogs.len()) as i32;
        match animal.get_type() {
            AnimalType::Dog => self.dogs.push(AnimalToAdopt {
                queued_timestamp: number_of_animals,
                animal,
            }),
            AnimalType::Cat => self.cats.push(AnimalToAdopt {
                queued_timestamp: number_of_animals,
                animal,
            }),
        }
    }

    fn dequeue_cat(&mut self) -> Option<Box<dyn Animal>> {
        if self.cats.is_empty() {
            None
        } else {
            Some(self.cats.remove(0).animal)
        }
    }

    fn dequeue_dog(&mut self) -> Option<Box<dyn Animal>> {
        if self.dogs.is_empty() {
            None
        } else {
            Some(self.dogs.remove(0).animal)
        }
    }

    fn dequeue_any(&mut self) -> Option<Box<dyn Animal>> {
        let cat_to_adopt = if self.cats.is_empty() {
            None
        } else {
            Some(self.cats.remove(0))
        };
        let dog_to_adopt = if self.dogs.is_empty() {
            None
        } else {
            Some(self.dogs.remove(0))
        };

        #[allow(clippy::unnecessary_unwrap)]
        if cat_to_adopt.is_some() && dog_to_adopt.is_some() {
            if cat_to_adopt.as_ref().unwrap().queued_timestamp
                < dog_to_adopt.as_ref().unwrap().queued_timestamp
            {
                Some(cat_to_adopt.unwrap().animal)
            } else {
                Some(dog_to_adopt.unwrap().animal)
            }
        } else if let Some(cat) = cat_to_adopt {
            Some(cat.animal)
        } else if let Some(dog) = dog_to_adopt {
            Some(dog.animal)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Animal, AnimalShelter, AnimalType, Cat, Dog};
    use rstest::{fixture, rstest};

    #[fixture]
    fn animal_shelter() -> AnimalShelter {
        let mut animal_shelter: AnimalShelter = AnimalShelter::default();
        animal_shelter.enqueue(Box::new(Cat::new(1)));
        animal_shelter.enqueue(Box::new(Cat::new(2)));
        animal_shelter.enqueue(Box::new(Dog::new(3)));
        animal_shelter.enqueue(Box::new(Dog::new(4)));
        animal_shelter
    }

    #[rstest]
    fn test_dequeue_any(animal_shelter: AnimalShelter) {
        let mut mut_animal_shelter = animal_shelter;
        let animal = mut_animal_shelter.dequeue_any().unwrap();

        assert_eq!(1, animal.get_id());
        assert_eq!(AnimalType::Cat, animal.get_type());
    }

    #[rstest]
    fn test_dequeue_dog(animal_shelter: AnimalShelter) {
        let mut mut_animal_shelter = animal_shelter;
        let animal = mut_animal_shelter.dequeue_dog().unwrap();

        assert_eq!(3, animal.get_id());
        assert_eq!(AnimalType::Dog, animal.get_type());
    }

    #[rstest]
    fn test_dequeue_cat(animal_shelter: AnimalShelter) {
        let mut mut_animal_shelter = animal_shelter;
        let animal = mut_animal_shelter.dequeue_cat().unwrap();

        assert_eq!(1, animal.get_id());
        assert_eq!(AnimalType::Cat, animal.get_type());
    }
}

fn main() {
    let mut animal_shelter: AnimalShelter = AnimalShelter::default();
    animal_shelter.enqueue(Box::new(Cat::new(1)));
    animal_shelter.enqueue(Box::new(Cat::new(2)));
    animal_shelter.enqueue(Box::new(Dog::new(3)));
    animal_shelter.enqueue(Box::new(Dog::new(4)));
    println!("{:?}", animal_shelter);
    println!("{:?}", animal_shelter.dequeue_dog().unwrap().get_id());
    println!("{:?}", animal_shelter.dequeue_any().unwrap().get_id());
    println!("{:?}", animal_shelter.dequeue_cat().unwrap().get_id());
}
