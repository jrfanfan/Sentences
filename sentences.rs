use rand::seq::SliceRandom;

// This program  generates simple English sentences.

fn main() {
    let mut num = 0;
    while num < 4 {
        num += 1;
        let x = vec![1,2];
        let mut rng = rand::thread_rng();
        let _quantity = x.choose(&mut rng).unwrap();
    
        let y = vec!["present", "past", "future"];
        let mut rng = rand::thread_rng();  
        let _tense = y.choose(&mut rng).unwrap();
        {
            println!("{} {} {} {} (your word please!) ... (suggested word) {}. ",
            get_determiner(*_quantity),get_noun(*_quantity),
            get_verb(*_quantity, _tense.to_string()),get_preposition(),
            get_object().to_ascii_lowercase());
        }
        
    }
    

}

// This function Randomly choose and return a determiner.    
fn get_determiner(quantity: i32) -> String{
    if quantity == 1 {
        let x = vec!["A", "One", "The"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
    } else {
        let x = vec!["Some", "Many", "The"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
    }
    
}
// This function Return a randomly chosen noun.
fn get_noun(quantity: i32) -> String {
    if quantity == 1 {
        let x = vec!["bird", "boy", "car", "cat", "child", "dog", "girl", "man",
        "rabbit", "woman"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
    } else {
        let x = vec!["birds", "boys", "cars", "cats", "children", "dogs",
        "girls", "men", "rabbits", "women"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
       
    }
    
}
// This function Return a randomly chosen verb. It will return one of these ten verbs:
fn get_verb(quantity: i32, tense: String) -> String {
    if tense == "past" {
        let x = vec!["drank", "ate", "grew", "laughed", "thought",
        "ran", "slept", "talked", "walked", "wrote"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
    } else if tense == "present" && quantity == 1 {
        let x = vec!["drinks", "eats", "grows", "laughs", "thinks",
        "runs", "sleeps", "talks", "walks", "writes"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
        
    } else if tense == "present" && quantity > 1 {
        let x = vec!["drink", "eat", "grow", "laugh", "think",
        "run", "sleep", "talk", "walk", "write"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
        
    } else {
        // Future
        let x = vec!["will drink", "will eat", "will grow", "will laugh",
        "will think", "will run", "will sleep", "will talk",
        "will walk", "will write"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
        
    }
}
// This function Return a randomly chosen preposition. It will return one of these prepositions:
fn get_preposition() -> String {
    let x = vec!["about", "above", "across", "after", "along",
        "around", "at", "before", "behind", "below",
        "beyond", "by", "despite", "except", "for",
        "from", "in", "into", "near", "of",
        "off", "on", "onto", "out", "over",
        "past", "to", "under", "with", "without"];
        let mut rng = rand::thread_rng();
        let choice = x.choose(&mut rng).unwrap();
        choice.to_string()
    
}
// This function Return a randomly chosen object. It will return one of these objects:
fn get_object() -> String {
    let x = vec!["Milk", "Butter", "Eggs", "Bread", "Cupboard", "Pillow",
    "Coffee", "maker", "Bed", "Spoon", "Blanket", "Knife", "Stove", "Sink",
    "Washing", "machine", "Pot", "Dish", "Fridge", "Sofa", "Stool", "Cup",
    "Fork", "Glass", "Pen", "Computer", "Notebook", "Desk", "Pencil","Bookcase",
    "Book", "Chair"," Backpack", "Paper", "Glue", "Door", "Ruler", "Clock",
    "Whiteboard", "Window"];
    let mut rng = rand::thread_rng();
    let choice = x.choose(&mut rng).unwrap();
    choice.to_string()
}
