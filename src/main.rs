use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::iter::Iterator;
use std::io;
use std::ffi::OsString;


fn main() {
    println!("Parsing...");    
    let name = get_file_argument();
    let instructions : Vec<Instruction> = parse(&get_file_contents(&name));

    println!("Running program with {} instructions.", instructions.len());
    fck(&instructions);
    println!("\nDone running!");
}

/// Get the first argument (which should be a file).
///
/// Avoids using a more complex argument parser, since this is good enough.
fn get_file_argument() -> OsString {
    match env::args_os().nth(1) {
        Some(x) => x,
        None => panic!("No file argument given."),
    }
}

/// Get the contents of the file specified by the string.
fn get_file_contents(name: &OsString) -> String{
    let fpath = Path::new(&name);
    // println!("Path specified is {:?}", fpath);
    if !fpath.is_file() { panic!("Path did not specify a file.") }
    let file = match File::open(fpath) {
        Ok(f) => f,
        Err(why) => panic!("Could not open file: {}.", why),
    };
    let mut reader = BufReader::new(file);
    let mut instructionsymbols:String = String::new();
    match reader.read_to_string(&mut instructionsymbols) {
        Result::Ok(y) => y,
        Result::Err(n) => panic!("Something went wrong during reading: {}.", n),   
    };
    instructionsymbols
}

/// Turn the symbols into a vector of instructions.
fn parse(symbols: &String) -> Vec<Instruction> {
    (*symbols).chars().filter_map(|x| match x {
            '+' => Some(Instruction::Add),
            '-' => Some(Instruction::Subtract),
            '>' => Some(Instruction::Next),
            '<' => Some(Instruction::Previous),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::GetInput),
            '[' => Some(Instruction::BeginLoop),
            ']' => Some(Instruction::EndLoop),
            _ => None,
        }).collect()
}

/// The basic Brainfuck instruction set.
#[derive(Debug)]
enum Instruction {
    /// Add one to the current cell.
    ///
    /// Wraps around instead of panic!-ing.
    Add,
    /// Subtract one off the current cell.
    ///
    /// Wraps around instead of panic!-ing.
    Subtract,
    /// Go to the next cell.
    ///
    /// Size is 60000, starting at 30000.
    /// Can therefore at most go 29999 to the right before panic!-ing.
    Next,
    /// Go to the previous cell.
    ///
    /// Size is 60000, starting at 30000.
    /// Can therefore at most go 30000 to the left before panic!-ing.
    Previous,
    /// Write the current value to the output stream.
    Output,
    /// Get input from the user, currently takes the last output.
    ///
    /// Unlike normal brainfuck implementations, this implementation takes the last character outputted instead.
    /// Getting input is as of writing this not implemented yet.
    GetInput,
    /// Start a while loop
    ///
    /// Takes into account the value at the current pointer, skips loop if 0.
    BeginLoop,
    /// End a loop.
    ///
    /// Goes back to start if value inequal to 0, goes to next instruction if equal.
    EndLoop,
}

/// Write an u8 to the main output.
fn putchar(ch: u8) {
    match io::stdout().write(&[ch]) {
        Ok(_) => {},
        Err(_) => panic!("Could not put char")
    }
}


/// Runs a brainfuck program as according to instructions as specified in the enum `Instruction`.
///
/// A few things to keep in mind:
/// The amount of elements is 60000, which is usually large enough for most programs.
/// In case of an overflow, you'll have to increase this number.
/// The element increment and decrement are implemented with a wrapping operation,
/// allowing some specific programs that rely on this behaviour to work.
///
/// This method does NOT optimise anything and runs the program basically as-is.
fn fck(instructions: &Vec<Instruction>) {
    let mut ipointer:usize = 0;
    let mut epointer:usize = 30000;
    let mut lo:u8 = 0;
    let mut elements:[u8; 60000] = [0; 60000];
    let mut pipointer:Vec<usize> = Vec::new();
    
    while ipointer < instructions.len() {
        match instructions[ipointer] {
            Instruction::Add            => { elements[epointer] = elements[epointer].wrapping_add(1); },
            Instruction::Subtract       => { elements[epointer] = elements[epointer].wrapping_sub(1); },
            Instruction::Next           => { epointer += 1; },
            Instruction::Previous       => { epointer -= 1; },
            Instruction::Output         => { putchar(elements[epointer]) ;
                                            lo = elements[epointer];},
            Instruction::GetInput       => { elements[epointer] = lo; },
            Instruction::BeginLoop      => { if elements[epointer] != 0 { pipointer.push(ipointer); } 
                                            else {
                                                // Find the matching ] bracket and use its position instead.
                                                ipointer = match instructions.iter().
                                                skip(ipointer + 1). //Start at ipointer.
                                                scan(1, |state:&mut u16, x| { match *x {  // Start at 1 with counting brackets
                                                    Instruction::BeginLoop => {*state += 1} // Add one for [
                                                    Instruction::EndLoop   => {*state -= 1} // Subtract one for ]
                                                    _                      => {}            // Ignore others
                                                    };
                                                    Some(*state) }). // Return the state at the end of each loop.
                                                position(|x:u16| match x { 0 => true, _ => false}) // Find the position of the first zero.
                                                {
                                                    Some(p) => p + ipointer + 1, // Position found, add ipointer since we skipped ipointer amount.
                                                    None    => panic!("Unmatched brackets for position {}!", ipointer) // Unmatched brackets. Oh noes.
                                                }
                                            } 
                                            }
            Instruction::EndLoop        => { if elements[epointer] == 0 { pipointer.pop(); } else { ipointer = *pipointer.last().expect("Unbalanced brackets!"); } },
        }
        ipointer += 1;
    }
}