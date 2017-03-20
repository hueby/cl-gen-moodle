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

    let mut question_type = String::new();
    get_input(&mut question_type);

    match question_type.trim() {
        "1" => println!("Single Choice"),
        "2" => println!("Multiple Choice"),
        "3" => println!("Essay"),
        "4" => println!("True/False"),
        "5" => println!("Matching"),
        "6" => println!("Short Answer"),
        "7" => println!("NumericalResponse"),
        _ => println!("else"),
    }

    println!("Type in the question text");
    let mut question_text = String::new();
    get_input(&mut question_text);

    println!("You typed: {}", question_text);

    let mut answers: Vec<Answer> = Vec::new();
    loop {
        println!("Insert Answer {}: [0] to exit", answers.len() + 1);
        let mut answer = String::new();
        get_input(&mut answer);
        println!("{}", answer);
        match answer.trim() {
            "0" => break,
            _ => {
                answers.push(Answer {
                    title: answer.to_string(),
                    points: 0,
                })
            }
        }
    }

    for answer in &answers {
        println!("{}", answer.title);
    }
}

fn get_input(input: &mut String) -> () {
    let mut read_line = String::new();
    match io::stdin().read_line(&mut read_line) {
        Ok(n) => {
            if n > 0 {
                *input = read_line;
            }
        }
        Err(error) => println!("error: {}", error),
    }
}

fn new_question(question_type: &str, name: &str) {
    println!("{}", question_type);
    println!("{}", name);
}
