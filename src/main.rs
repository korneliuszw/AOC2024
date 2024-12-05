pub mod defs;
mod solutions;
use defs::Solution;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let solution_number = args[1].parse::<u32>().unwrap();
    let part = args[2].parse::<u32>().unwrap();
    let solver: Box<dyn Solution> = match solution_number {
        1 => Box::new(solutions::sol1::Solution1 {}),
        2 => Box::new(solutions::sol2::Solution2 {}),
        3 => Box::new(solutions::sol3::Solution3 {}),
        4 => Box::new(solutions::sol4::Solution4 {}),
        5 => Box::new(solutions::sol5::Solution5 {}),
        _ => {
            panic!("Invalid solution number");
        }
    };
    let input_file = format!("inputs/{}.in", solution_number);
    let output_file = format!("outputs/{}.out", solution_number);
    let mut reader = std::io::BufReader::new(std::fs::File::open(input_file).unwrap());
    let writer = &mut std::io::BufWriter::new(std::fs::File::create(output_file).unwrap());
    solver.solve(&mut reader, writer, part);
}
