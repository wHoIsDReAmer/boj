use std::io::{self, BufRead};

enum Rate {
    APlus = 45,
    AZero = 40,
    BPlus = 35,
    BZero = 30,
    CPlus = 25,
    CZero = 20,
    DPlus = 15,
    DZero = 10,
    P = 1,
    F = 0,
}

impl From<&str> for Rate {
    fn from(value: &str) -> Self {
        match value {
            "A+" => Rate::APlus,
            "A0" => Rate::AZero,
            "B+" => Rate::BPlus,
            "B0" => Rate::BZero,
            "C+" => Rate::CPlus,
            "C0" => Rate::CZero,
            "D+" => Rate::DPlus,
            "D0" => Rate::DZero,
            "P" => Rate::P,
            _ => Rate::F,
        }
    }
}

impl Rate {
    fn to_f32(self) -> f32 {
        self as i32 as f32 / 10.0
    }
}

fn main() {
    let mut io_lock = io::stdin().lock();
    let mut total: f32 = 0.0;
    let mut total_grade: f32 = 0.0;
    
    for _ in 0..20 {
        let mut _buf = String::new();
        io_lock.read_line(&mut _buf).unwrap();

        let inputs: Vec<&str> = _buf.split_whitespace().collect();
        let grade = inputs[1].parse::<f32>().unwrap();
        let rate = Rate::from(inputs[2]);

        match rate {
            Rate::P => {
                continue
            }
            _ => {
                total_grade += grade;
            }
        }
        total += rate.to_f32() * grade;
    }

    println!("{:.6}", total / total_grade);
}
