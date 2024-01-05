use std::io;

struct Animal {
    name: String,
    species: String,
}

impl Animal {
    fn new(name: String, species: String) -> Animal {
        Animal { name, species }
    }

    fn display(&self) {
        println!("Name: {}\nSpecies: {}", self.name, self.species);
    }
}

fn main() {
    let mut animals: Vec<Animal> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Add an animal");
        println!("2. Display all animals");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter name of the animal:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                println!("Enter species of the animal:");
                let mut species = String::new();
                io::stdin().read_line(&mut species).expect("Failed to read line");

                let animal = Animal::new(name.trim().to_string(), species.trim().to_string());
                animals.push(animal);
            }
            2 => {
                println!("All Animals:");
                for animal in &animals {
                    animal.display();
                }
            }
            3 => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice, please choose a valid option."),
        }
    }
}