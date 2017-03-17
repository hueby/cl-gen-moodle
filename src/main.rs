use std::io;
use std::vec::Vec;

/**
 *
 * - struct for all question types
 * - input routine
 * - hash set as data structure
 * - export function
 *
 */

struct Answer {
    title: String,
    points: i32,
}

struct Question {
    question_type: QuestionType,
    title: String,
    answers: Vec<Answer>,
}

enum QuestionType {
    SingleChoice,
    MultipleChoice,
    Essay,
    TrueFalse,
    Matching,
    ShortAnswer,
    NumericalResponse,
}

fn main() {
    println!("Bitte Fragetyp eingeben: [1-7], 0 for help");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {}
        Err(error) => println!("error: {}", error),
    }

    if input == "1" {
        println!("SingleChoice");
    }
    println!("{}", input);
}

fn new_question(question_type: &str, name: &str) {
    println!("{}", question_type);
    println!("{}", name);
}
