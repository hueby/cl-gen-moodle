use std::io;
use std::vec::Vec;

mod lib;
use lib::xml_export::QuestionType;
use lib::xml_export::Question;
use lib::xml_export::Answer;

/**
 *
 * - struct for all question types (done)
 * - input routine (done)
 * - vector as main data structure (done)
 * - export function
 * - input validation
 *
 */

fn main() {
    let mut questions: Vec<Question> = Vec::new();

    new_checklist(&mut questions);

    lib::xml_export::questions_to_moodle(&mut questions);
}

fn new_checklist(questions: &mut Vec<Question>) {
    // Ask the question type
    // 2 or more answers
    // next question? yes loop again, no break

    loop {
        println!("Bitte Fragetyp eingeben: [1-7] ");

        let mut question_type_raw = String::new();
        get_input(&mut question_type_raw);

        let question_type: QuestionType = get_question_type(question_type_raw);

        println!("Type in the question text:");
        let mut question_text = String::new();
        get_input(&mut question_text);

        let mut answers: Vec<Answer> = Vec::new();
        add_answers(&mut answers, &question_type);

        let question = Question {
            question_type: question_type,
            title: question_text,
            answers: answers,
        };

        questions.push(question);

        println!("Next Question? [J/n]: ");
        let mut next_question = String::new();
        get_input(&mut next_question);
        match next_question.trim() {
            "J" => continue,
            "n" => break,
            "\n" => continue,
            _ => break,
        }
    }

}

fn add_answers(answers: &mut Vec<Answer>, question_type: &QuestionType) {
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
                });
                match *question_type {
                    QuestionType::SingleChoice => continue,
                    QuestionType::MultipleChoice => continue,
                    _ => break,
                }
            }
        }
    }
}

fn get_input(input: &mut String) -> () {
    let mut read_line = String::new();
    match io::stdin().read_line(&mut read_line) {
        Ok(n) => {
            if n > 0 {
                read_line.pop();
                *input = read_line;
            }
        }
        Err(error) => println!("error: {}", error),
    }
}

fn get_question_type(question_type_raw: String) -> QuestionType {
    match question_type_raw.trim() {
        "1" => QuestionType::SingleChoice,
        "2" => QuestionType::MultipleChoice,
        "3" => QuestionType::Essay,
        "4" => QuestionType::TrueFalse,
        "5" => QuestionType::Matching,
        "6" => QuestionType::ShortAnswer,
        "7" => QuestionType::NumericalResponse,
        _ => QuestionType::UndefinedQuestion,
    }
}
