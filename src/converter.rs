use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

struct Swish {
    name: String,
    amount: String,
}

pub fn convert(filename: String) {
    let mut readfile = BufReader::new(File::open(filename).unwrap());
    let mut writefile = BufWriter::new(File::create("swish.csv")
        .expect("Failed to create swish.csv"));


    // write_to_csv(&mut writefile, "Name".to_string(), "Amount".to_string());

    let mut buf = String::new();

    while let Ok(line) = readfile.read_line(&mut buf) {
        if line == 0 {
            break;
        }
        let swish = get_data(buf.clone());
        if swish.is_none() {
            buf.clear();
            continue;
        }
        write_to_csv(&mut writefile, swish.unwrap());
        buf.clear();
    }


    writefile.flush().unwrap();
}

/**
 * convert
 *  swish_ocr/swish1.jpg.txt:Firstname Lastname 12 kr
 * to
 *  Firstname Lastname 12
 */
fn get_data(raw_data: String) -> Option<Swish> {
    let rd = raw_data.split(":");
    let mut data = Vec::new();
    for s in rd {
        data.push(s.to_string());
    }
    data = data[1..].to_vec();
    let mut data = data
        .concat()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    data.pop();
    if !validate_data(data.clone()) {
        // panic!("Invalid data: {:?}", data);
        return None;
    }

    let amount = data.pop().unwrap();
    let name = data.join(" ");

    return Some(Swish {
        name,
        amount,
    });
}

fn validate_data(data: Vec<String>) -> bool {
    if data.len() < 2 {
        return false;
    }
    let last = data.last().unwrap();
    
    if last.parse::<i32>().is_err() { // check if the last element of data is a number
        return false;
    }
    return true;
}

fn write_to_csv(w: &mut BufWriter<File>, swish: Swish) {
    let line = format!("{},{}\n", swish.name, swish.amount);
    w.write(line.as_bytes()).unwrap();
}