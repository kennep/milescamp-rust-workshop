enum SeaAnimal {
    Fish,
    Crab,
    SeaCucumber,
    Oyster
}

fn make_joke(animal: SeaAnimal) -> Result<&'static str, &'static str> {
    match animal {
        SeaAnimal::Fish => Ok("What do you call a fish in a tie?\nSofishticated."),
        SeaAnimal::Crab => Ok("Why did the crab cross the road?\nIt didn't. It used the sidewalk."),
        SeaAnimal::SeaCucumber => Err("I don't know any sea cucumber jokes"),
        SeaAnimal::Oyster => Ok("You know why oysters keep everything to themselves?\nBecause they're shellfish.")
    }
}

fn main() {
    match make_joke(SeaAnimal::Oyster) {
        Ok(joke) => print!("Do you want to hear a joke? Here it is:\n\n{}", joke),
        Err(error) => print!("Could not find joke:\n\n{}", error)
    }
}
