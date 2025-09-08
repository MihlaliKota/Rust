fn main() {
    let my_integer = 5;
    let my_float = 3.14;
    let is_learning_rust = true;
    let favorite_letter = 'M';
    let my_scores: [i32; 5] = [58, 65, 43, 24, 92];
    let hobby = "reading manhwa.";

    println!("Favorite number: {}", my_integer);
    println!("Second favorite number: {}", my_float);
    println!("Is he learning rust?: {}", is_learning_rust);
    println!("Favorite letter: {}", favorite_letter);
    println!("My scores: {:?}", my_scores);
    println!("I enjoy {}", hobby);
}