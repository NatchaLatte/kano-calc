use std::io::{self, Write};

enum JobType {
    ReadyMade,
    Customized,
    CustomMade,
}

impl JobType {
    fn from_str(input: &str) -> Option<JobType> {
        match input.trim().to_lowercase().as_str() {
            "1" => Some(JobType::ReadyMade),
            "2" => Some(JobType::Customized),
            "3" => Some(JobType::CustomMade),
            _ => None,
        }
    }
}

struct Job {
    job_type: JobType,
    price: f64,
    rated_five_star: bool,
}

impl Job {
    fn calculate_wage(&self) -> u32 {
        let base_rate = match self.job_type {
            JobType::ReadyMade => 0.50,
            JobType::Customized => 0.60,
            JobType::CustomMade => 0.70,
        };

        let bonus_rate = if self.rated_five_star { 0.05 } else { 0.0 };
        let total_rate = base_rate + bonus_rate;

        (self.price * total_rate).ceil() as u32
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== โปรแกรมคำนวณค่าแรงร้านสกิน Minecraft ===");

    // อ่านประเภทงาน
    let job_type = loop {
        println!("ประเภทงานทั้งหมด");
        println!("(1) Ready-Made");
        println!("(2) Customized");
        println!("(3) Custom-Made");
        let input = read_line("กรุณากรอกตัวเลขประเภทงาน: ");
        if let Some(jt) = JobType::from_str(&input) {
            break jt;
        } else {
            println!("ไม่พบประเภทงาน กรุณาลองใหม่");
        }
    };

    // อ่านราคางาน
    let price = loop {
        let input = read_line("กรุณาใส่ราคางาน (บาท): ");
        match input.parse::<f64>() {
            Ok(val) if val > 0.0 => break val,
            _ => println!("กรุณาใส่จำนวนเงินที่ถูกต้อง (มากกว่า 0)"),
        }
    };

    // อ่านว่าลูกค้าให้ 5 ดาวไหม
    let rated_five_star = loop {
        let input = read_line("ลูกค้าให้ 5 ดาวไหม? (y/n): ");
        match input.to_lowercase().as_str() {
            "y" | "yes" => break true,
            "n" | "no" => break false,
            _ => println!("กรุณาใส่ y หรือ n"),
        }
    };

    let job = Job {
        job_type,
        price,
        rated_five_star,
    };

    let wage = job.calculate_wage();
    println!("ค่าแรงลูกจ้างที่ต้องจ่าย: {} บาท", wage);
}
