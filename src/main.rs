
use text_colorizer::*;
use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct Arguments{
    pattern:String,
    replace:String,
    input_file:String,
    output_file:String,
}



fn main() {
    print_help_message();
}

fn print_help_message(){
    eprintln!("{} - replace a string contained inside a file, with a new string","Find and replace".green());
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn setup(){
        assert_eq!(2,2);
    }
    #[test]
    fn test_read_external_args(){
        use std::io::{BufRead, BufReader, BufWriter, Write};
        use std::process::{Command, Stdio};

        let a = vec!["view", "-h", "file.bam"];
        let mut b=vec![];

        let mut child = Command::new("find_and_replace")
            .args(&a)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

            if let Some(ref mut stdout) = child.stdout {
                for line in BufReader::new(stdout).lines() {
        
                    let mut l: String = line.unwrap();
                    // Need to add an end of line character back to the string
                    let eol: &str = "\n";
                    l = l + eol;
        
                   b.push(l);
            }
        }
        assert_eq!("",b[0]);


    }
    
}