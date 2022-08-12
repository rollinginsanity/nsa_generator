use rand::seq::SliceRandom;
fn main() {
    let animals_bytes = include_bytes!("animals.txt");
    let adjectives_bytes = include_bytes!("adjectives.txt");

    let animals = String::from_utf8_lossy(animals_bytes);
    let adjectives = String::from_utf8_lossy(adjectives_bytes);

    let mut split_animals = animals.split("\n");
    let mut split_adjectives = adjectives.split("\n");

    let animals_arr: Vec<&str> = split_animals.collect();
    let adjectives_arr: Vec<&str> = split_adjectives.collect();

    let animal = animals_arr.choose(&mut rand::thread_rng());
    let adjective = adjectives_arr.choose(&mut rand::thread_rng());


    println!("{}{}", Option::expect(adjective, "test").to_uppercase(), Option::expect(animal, "test1").to_uppercase());
}
