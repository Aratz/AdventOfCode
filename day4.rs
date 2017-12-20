fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut number_correct = 0;

    for passphrase in stdin.lock().lines(){
        use std::collections::BTreeSet;
        let mut word_set = BTreeSet::new();
        let mut correct = true;

        for word in passphrase.unwrap().split(" "){
            if !word_set.contains(&String::from(word)){
                word_set.insert(String::from(word));
            }
            else{
                correct = false;
                break;
            }
        }
        if correct {
            number_correct += 1;
        }
    }
    println!("{}", number_correct);


}
