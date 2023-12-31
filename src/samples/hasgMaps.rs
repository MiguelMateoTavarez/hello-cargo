use std::collections::HashMap;

fn main() {
    //*Hash map */

    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    //?Get value
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\':{:?}", book, reviews.get(book));

    //?Delete value
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
