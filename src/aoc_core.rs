use regex::Regex;
use std::{
    env,
    fmt::{Debug, Display},
    fs,
    path::Path,
    str::FromStr,
    sync::LazyLock,
    time::Instant,
};

pub fn read_file(relative_to: &Path, name: &str) -> String {
    let dir: &Path;
    if relative_to.is_file() {
        dir = relative_to.parent().unwrap();
    } else {
        dir = relative_to;
    }

    let input_file_path = dir.join(name);
    let contents = fs::read_to_string(input_file_path).expect("Input file should be there");
    return contents;
}

pub type AocResult = Result<i64, String>;

pub trait AocTask {
    fn solve_a(&self, contents: String) -> AocResult;
    fn solve_b(&self, contents: String) -> AocResult;
}

#[derive(Debug)]
pub enum TaskPart {
    A,
    B,
}

impl Default for TaskPart {
    fn default() -> Self {
        return TaskPart::A;
    }
}
impl Display for TaskPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            TaskPart::A => "A",
            TaskPart::B => "B",
        };
        f.write_str(val)
    }
}

impl FromStr for TaskPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "a" => Ok(TaskPart::A),
            "b" => Ok(TaskPart::B),
            _ => Err(String::from("invalid task part")),
        }
    }
}

const ARG_INDEX_DAY: usize = 0;
const ARG_INDEX_PART: usize = 1;
const ARG_INDEX_INPUT: usize = 2;

#[derive(Debug)]
struct AocArgs {
    day_number: usize,
    part: TaskPart,
    input: String,
}

static DAY_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:day(\d+)|(\d+))").unwrap());

impl AocArgs {
    fn parse_day_arg(s: &String) -> Result<usize, String> {
        let captures = DAY_PATTERN
            .captures(s.as_str())
            .ok_or("invalid argument for day")?;

        let day_raw = captures.get(1).or(captures.get(2)).unwrap();
        day_raw.as_str().parse::<usize>().map_err(|e| e.to_string())
    }

    fn parse_args(n_days_implemented: usize) -> Result<AocArgs, String> {
        let args: Vec<String> = env::args().skip(1).collect();

        let day_number = args
            .get(ARG_INDEX_DAY)
            .ok_or(String::from("missing day parameter"))
            .and_then(Self::parse_day_arg)?;
        if day_number - 1 >= n_days_implemented {
            return Err(format!("day {} is not yet implemented", day_number));
        }

        let task_part = args
            .get(ARG_INDEX_PART)
            .map(|r| r.parse::<TaskPart>())
            .unwrap_or(Ok(TaskPart::default()))?;

        let input_type = args
            .get(ARG_INDEX_INPUT)
            .map(String::as_str)
            .unwrap_or("sample");
        let input_file = match input_type.to_lowercase().as_str() {
            "sample" | "s" => "sample.txt",
            "input" | "i" => "input.txt",
            _ => input_type,
        }
        .to_owned();
        Ok(AocArgs {
            day_number,
            part: task_part,
            input: input_file,
        })
    }

    fn help() {
        println!("cargo run <day> [part] [input]")
    }
}

pub fn run<D: IntoIterator<Item = Box<dyn AocTask>>>(days: D) {
    let days = days.into_iter().collect::<Vec<_>>();
    let args = AocArgs::parse_args(days.len());
    if args.is_err() {
        println!("{}", args.unwrap_err());
        AocArgs::help();
        panic!()
    }

    let args = args.unwrap();

    println!(
        "Evaluating day {} / {} for input {}",
        args.day_number, args.part, args.input
    );
    let day = days.get(args.day_number - 1).unwrap();

    let contents = read_file(
        Path::new(file!())
            .parent()
            .unwrap()
            .join(format!("day{:0>2}", args.day_number))
            .as_path(),
        args.input.as_str(),
    );

    let solver: Box<dyn Fn(String) -> AocResult> = match args.part {
        TaskPart::A => Box::new(|c| day.solve_a(c)),
        TaskPart::B => Box::new(|c| day.solve_b(c)),
    };

    let now = Instant::now();

    let result = solver(contents);
    let elapsed = now.elapsed();
    println!("Ran {:?}", elapsed);

    match result {
        Ok(value) => {
            println!("Result: {}", value);
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
