///
/// Command line app. to generate and validate a swiss navs13.
/// This utility is for testing purposes, only the structure is validated which does not mean
/// that the number is administratively valid.
/// https://www.zas.admin.ch/zas/fr/home/partenaires-et-institutions-/navs13.html
/// Algorithm is describe in annexe 8 :
/// https://sozialversicherungen.admin.ch/fr/d/6938#
///
use std::char;
use std::fmt::{Display, Formatter};
use std::process::exit;
use std::str::FromStr;
use rand::prelude::*;
use clap::{Parser,ArgGroup};

/// NAVS with 13 digits
#[derive(Debug)]
struct Nav13 {
    digits : [u8;12],
    check: u8
}

/// Format an Nav13
impl Display for Nav13 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}{}.{}{}{}{}.{}{}{}{}.{}{}",
               &self.digits[0],
               &self.digits[1],
               &self.digits[2],
               &self.digits[3],
               &self.digits[4],
               &self.digits[5],
               &self.digits[6],
               &self.digits[7],
               &self.digits[8],
               &self.digits[9],
               &self.digits[10],
               &self.digits[11],
               &self.check
        )
    }
}

/// Convert from string a Nav13
impl From<String> for Nav13 {
    fn from(value: String) -> Self {
        let nav13 = Nav13::from_str(value.as_str());
        nav13.expect("Expect a valid NAVS13.")
    }
}

/// Parse error
#[derive(Debug, PartialEq, Eq)]
struct ParseNav13Error {
    code : i32,
    description: String,
}

/// Convert from str a Nav13
impl FromStr for Nav13 {
    type Err = ParseNav13Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nav13 : Nav13 = Nav13 {
            digits : [255;12],
            check : 255,
        };
        // extract digits
        let values : Vec<u8> = s.chars()
            .filter_map(|c| {
                match c.is_numeric(){
                    true => Some(char::to_digit(c,10).unwrap() as u8),
                    _ => None
                }
            }).collect();
        // Validate
        if values.len() != 13 {
            let description = format!("Number of digits should be 13. Found: {}",values.len());
            let e = ParseNav13Error {
                code : 64,
                description,
            };
            return Err(e);
        }
        if values[0]!=7 && values[1]!=5 && values[2]!=6 {
            let description = format!("{}{}{} isn't iso-3166 for Switzerland.",values[0], values[1], values[2]);
            let e = ParseNav13Error {
                code : 65,
                description,
            };
            return Err(e);
        }
        // Copy into Nav13
        nav13.check = values[12];
        values[0..12].iter().zip(nav13.digits.iter_mut()).map(|(&x, y)| *y = x).count();
        // Validate EAN13
        if nav13.check != ean13_check(&nav13.digits) {
            let description = format!("{} Is invalid EAN-13.",nav13.check);
            let e = ParseNav13Error {
                code : 66,
                description,
            };
            return Err(e);
        }
        Ok(nav13)
    }
}

/// Compute check of digits
fn ean13_check(digits : &[u8;12]) -> u8{
    // Compute EAN-13 check
    let mut s : u16 = 0; //sum
    // for each digits
    for d in digits.iter().enumerate()  {
        // Determine weight
        let p :u16 = match d.0 % 2 {
            0 => 1,
            1 => 3,
            _ => unreachable!(),
        };
        s += *d.1 as u16 * p;
    }
    match s % 10 {
        0 => 0,
        _ => (10 - (s % 10)) as u8
    }
}

/// create a navs13
fn create_fake_swiss_navs13() -> Nav13{
    let mut rng = thread_rng();
    let mut digits: [u8;12] = [255;12];
    // add swiss iso-3166-1-num-3 CH=756
    digits[0]=7;
    digits[1]=5;
    digits[2]=6;
    // add random value between 0 - 9
    let mut i = 3;
    while i < 12 {
        digits[i]=rng.gen_range(0..10);
        i += 1;
    }
    // Compute EAN13 check key
    let check = ean13_check(&digits);

    Nav13{
        digits,
        check,
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about)]
#[command(next_line_help = true)]
#[command(arg_required_else_help(true))]
#[command(group(
ArgGroup::new("validate")
.args(["navs13"])
.conflicts_with_all(["create","number"]),
))]
struct Cli {
    #[arg(help = "NAVS13 to validate.\nNote that only the structure is validated. This is not enough to make it effective.\nExample: 756.1234.5678.97")]
    navs13: Option<String>,

    // Flag parameters
    #[arg(short, long ,help = "Creates a structurally valid navs13 for test purposes.")]
    create: bool,

    #[arg(short, long ,help = "Number of NAVS13 to generate. max=255",default_value_t = 1)]
    number: u8,

}

fn main() {
    let cli = Cli::parse();

    if cli.create {
        for _x in 0..cli.number {
            println!("{}",create_fake_swiss_navs13());
        }
        exit(0)
    }

    if let Some(navs13) = cli.navs13 {
        let n = Nav13::from_str(navs13.as_str());
        match n {
            Ok(n) => {
                println!("{} is valid.", n);
                exit(0)
            }
            Err(e) => {
                eprintln!("{} is invalid. Error code {}, description {}",navs13,e.code,e.description);
                exit(e.code)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{ean13_check, Nav13};

    #[test]
    fn test_ean13_check_7() {
        let digits: [u8;12] = [7,5,6,1,2,3,4,5,6,7,8,9];
        let c = ean13_check(&digits);
        assert_eq!(c,7,"The control key must be 7")
    }

    #[test]
    fn test_ean13_check_0() {
        let digits: [u8;12] = [7,5,6,4,9,6,5,7,6,6,5,6];
        let c = ean13_check(&digits);
        assert_eq!(c,0,"The control key must be 0")
    }

    #[test]
    fn test_ean13_check_9() {
        let digits: [u8;12] = [4,7,1,9,5,1,2,0,0,2,8,8]; //EAN13 Wikipedia
        let c = ean13_check(&digits);
        assert_eq!(c,9,"The control key must be 9")
    }

    #[test]
    fn test_from_str_ok() {
        let test = Nav13::from_str("756.2465.8935.64");
        assert!(test.is_ok(),"{}",test.err().unwrap().description)
    }

    #[test]
    fn test_from_str_nok_len_short() {
        let test = Nav13::from_str("756.246.8935.64");
        assert!(test.is_err());
        let e = test.err().unwrap();
        println!("Error {} - {}",e.code,e.description);
        assert_eq!(e.code,64)
    }

    #[test]
    fn test_from_str_nok_len_long() {
        let test = Nav13::from_str("756.246.8935.64789");
        assert!(test.is_err());
        let e = test.err().unwrap();
        println!("Error {} - {}",e.code,e.description);
        assert_eq!(e.code,64)
    }

    #[test]
    fn test_from_str_nok_iso() {
        let test = Nav13::from_str("471.9512.0028.88");
        assert!(test.is_err());
        let e = test.err().unwrap();
        println!("Error {} - {}",e.code,e.description);
        assert_eq!(e.code,65)
    }

    #[test]
    fn test_from_str_nok_ean() {
        let test = Nav13::from_str("756.2465.8935.65");
        assert!(test.is_err());
        let e = test.err().unwrap();
        println!("Error {} - {}",e.code,e.description);
        assert_eq!(e.code,66)
    }
}
