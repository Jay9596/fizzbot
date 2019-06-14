mod applicant;
use applicant::candidate::Interviewee;

fn main() {
    Interviewee::new("Rust").start_interview();
}
