extern crate csv;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::str::FromStr;

const SelfAssessmentStr: &str = "self-assessment";
const TeamFeedbackStr: &str = "team-feedback";

enum InAssessmentType {
    TeamFeedback,
    SelfAssessment,
}

impl InAssessmentType {
    fn questions_config(&self) -> Vec<u32> {
        match self {
            InAssessmentType::SelfAssessment => vec![2, 2, 3, 2, 3, 2, 3, 3, 2, 2, 3, 2],
            InAssessmentType::TeamFeedback => vec![2, 2, 3, 2, 3, 2, 4, 3, 2, 2, 2, 2],
        }
    }
}

impl std::str::FromStr for InAssessmentType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TeamFeedbackStr => Ok(InAssessmentType::TeamFeedback),
            SelfAssessmentStr => Ok(InAssessmentType::SelfAssessment),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "InAssessemntType parse error. valid types: `team-feedback`, `self-assessment`",
            )),
        }
    }
}

impl Display for InAssessmentType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InAssessmentType::SelfAssessment => write!(f, "self-assessment"),
            InAssessmentType::TeamFeedback => write!(f, "team-feedback"),
        }
    }
}

enum Skill {
    Adaptability,
    Attitude,
    Communication,
    CrossFunctionalKnowledge,
    Dependability,
    Initiative,
    Leadership,
    Organization,
    Responsibility,
    SelfImprovement,
    Teamwork,
    TechExpertise,
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Skill::Adaptability => write!(f, "Adaptability"),
            Skill::Attitude => write!(f, "Attitude"),
            Skill::Communication => write!(f, "Communication"),
            Skill::CrossFunctionalKnowledge => write!(f, "Cross-functional Knowledge"),
            Skill::Dependability => write!(f, "Dependability"),
            Skill::Initiative => write!(f, "Initiative"),
            Skill::Leadership => write!(f, "Leadership"),
            Skill::Organization => write!(f, "Organization"),
            Skill::Responsibility => write!(f, "Responsibility"),
            Skill::SelfImprovement => write!(f, "Self-Improvement"),
            Skill::Teamwork => write!(f, "Teamwork"),
            Skill::TechExpertise => write!(f, "Tech. Expertise"),
        }
    }
}

struct EmployeeSkill {
    name: Skill,
    questions: u32,
    grades: Vec<u32>,
}

impl EmployeeSkill {
    fn new(n: Skill, q: u32) -> EmployeeSkill {
        EmployeeSkill {
            name: n,
            questions: q,
            grades: Vec::with_capacity(q as usize),
        }
    }

    fn add_grade(&mut self, v: u32) {
        self.grades.push(v)
    }

    fn avg(&self) -> f32 {
        self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32
    }
}

impl fmt::Display for EmployeeSkill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.avg())
    }
}

fn main() {
    let (employee_name, feedback_type) = parse_flags().expect("could not parse input flags");
    let mut config_questions = feedback_type.questions_config().into_iter();

    let mut questions: Vec<EmployeeSkill> = vec![
        EmployeeSkill::new(Skill::Adaptability, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Attitude, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Communication, config_questions.next().unwrap()),
        EmployeeSkill::new(
            Skill::CrossFunctionalKnowledge,
            config_questions.next().unwrap(),
        ),
        EmployeeSkill::new(Skill::Dependability, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Initiative, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Leadership, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Organization, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Responsibility, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::SelfImprovement, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::Teamwork, config_questions.next().unwrap()),
        EmployeeSkill::new(Skill::TechExpertise, config_questions.next().unwrap()),
    ];

    let mut rdr = csv::Reader::from_reader(io::stdin());
    for (index, colleague) in rdr.records().enumerate() {
        let feedback = colleague.expect("could not read colleague's feedback");
        let mut feedback_iter = feedback.iter().enumerate();

        println!(">> scanning response: {}\n", index);

        for q in &mut questions {
            println!("scanning '{}'...", q.name);
            let mut counter: u32 = 0;

            loop {
                let (index, answer) = feedback_iter
                    .next()
                    .expect("could not retrieve the next value");

                if index < 2 {
                    // skip email and timestamp fields
                    continue;
                }

                let grade: u32 = answer.parse().expect("could not parse grade");
                q.add_grade(grade);

                // break to the next category
                counter = counter + 1;
                if counter == q.questions {
                    break;
                }
            }
        }
    }

    let target_filename = format!("{}_{}.csv", employee_name, feedback_type.to_string());
    let mut wrt = csv::WriterBuilder::new()
        .from_path(&target_filename)
        .unwrap();

    for i in 0..2 {
        for skill in &questions {
            if i == 0 {
                wrt.write_field(skill.name.to_string()).unwrap();
            } else {
                wrt.write_field(format!("{}", skill.avg())).unwrap();
            }
        }
        wrt.write_record(None::<&[u8]>).unwrap();
    }

    wrt.flush().unwrap();

    println!("\ngenerated file: {}", target_filename);
}

fn parse_flags() -> Result<(String, InAssessmentType), impl std::error::Error> {
    let mut employee_name = String::new();
    let mut assessment_type = String::new();

    // todo: remove panics
    std::env::args().for_each(|arg: String| {
        if arg.contains("-name=") {
            employee_name = arg
                .trim_start_matches("-name=")
                .parse::<String>()
                .expect("could not parse `-name` flag");
        }

        if arg.contains("-type=") {
            assessment_type = arg
                .trim_start_matches("-type=")
                .parse::<String>()
                .expect("could not parse `-type` flag");
        }
    });

    if employee_name.is_empty() || assessment_type.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "empty `-name` or `-type` flags provided",
        ))
    } else {
        let feedback_type = InAssessmentType::from_str(assessment_type.as_str());
        match feedback_type {
            Ok(t) => Ok((employee_name, t)),
            Err(err) => Err(err),
        }
    }
}
