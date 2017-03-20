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
#[derive(Debug)]
struct Answer {
    title: String,
    points: i32,
}

#[derive(Debug)]
struct Question {
    question_type: QuestionType,
    title: String,
    answers: Vec<Answer>,
}

#[derive(Debug)]
enum QuestionType {
    SingleChoice,
    MultipleChoice,
    Essay,
    TrueFalse,
    Matching,
    ShortAnswer,
    NumericalResponse,
    UndefinedQuestion,
}

fn main() {
    println!("Bitte Fragetyp eingeben: [1-7], 0 for help");

    let mut question_type_raw = String::new();
    let mut question_type: QuestionType = QuestionType::UndefinedQuestion;
    get_input(&mut question_type_raw);

    match question_type_raw.trim() {
        "1" => question_type = QuestionType::SingleChoice,
        "2" => question_type = QuestionType::MultipleChoice,
        "3" => question_type = QuestionType::Essay,
        "4" => question_type = QuestionType::TrueFalse,
        "5" => question_type = QuestionType::Matching,
        "6" => question_type = QuestionType::ShortAnswer,
        "7" => question_type = QuestionType::NumericalResponse,
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

    let question = Question {
        question_type: question_type,
        title: question_text,
        answers: answers,
    };

    println!("{:?}", question);
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
