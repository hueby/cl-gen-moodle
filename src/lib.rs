pub mod xml_export {

    use std::fs::File;
    use std::io::prelude::*;
    use std::error::Error;

    pub enum QuestionType {
        SingleChoice,
        MultipleChoice,
        Essay,
        TrueFalse,
        Matching,
        ShortAnswer,
        NumericalResponse,
        UndefinedQuestion,
    }
    pub struct Answer {
        pub title: String,
        pub points: i32,
    }
    pub struct Question {
        pub question_type: QuestionType,
        pub title: String,
        pub answers: Vec<Answer>,
    }

    pub fn questions_to_moodle(questions: &mut Vec<Question>) {
        let mut xml_string = String::new();
        xml_string += "<?xml version=\"1.0\"?>";
        xml_string += "<quiz>";

        for question in questions {
            let question_type_id = determine_type(&question.question_type);
            let new_question = format!("<question type=\"{}\">", question_type_id);
            xml_string += &new_question;

            xml_string += "<name><text>";
            xml_string += &question.title;
            xml_string += "</text></name>";
            xml_string += "<questiontext format=\"html\">";
            xml_string += "<text>";
            xml_string += &question.title;
            xml_string += "</text></questiontext>";

            for answer in &question.answers {
                xml_string += "<answer fraction=\"0\">";
                xml_string += "<text>";
                let answer_text = &answer.title;
                xml_string += &answer_text;
                xml_string += "</text>";
                xml_string += "</answer>";
            }

            xml_string += "</question>";
        }
        xml_string += "</quiz>";
        print_to_file(&xml_string);
    }

    fn print_to_file(xml_string: &String) {
        let mut file = match File::create("test.xml") {
            Err(why) => panic!("Ging net: {}", why.description()),
            Ok(file) => file,
        };

        match file.write_all(xml_string.as_bytes()) {
            Err(why) => {
                panic!("Ging auch net: {}", why.description());
            }
            Ok(_) => println!("Gesaved!"),
        }
    }

    fn determine_type(question_type: &QuestionType) -> String {
        match question_type {
            &QuestionType::SingleChoice => "multichoice".to_string(),
            &QuestionType::MultipleChoice => "multichoice".to_string(),
            &QuestionType::Essay => "essay".to_string(),
            &QuestionType::TrueFalse => "truefalse".to_string(),
            &QuestionType::Matching => "match".to_string(),
            &QuestionType::ShortAnswer => "shortanswer".to_string(),
            &QuestionType::NumericalResponse => "numerical".to_string(),
            &QuestionType::UndefinedQuestion => "description".to_string(),
        }
    }
}
