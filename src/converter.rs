use std::fs::File;
use std::io::{BufReader, Read, BufRead, BufWriter, Write};


pub fn convert(filename: String) {
    let mut readfile = BufReader::new(File::open(filename).unwrap());
    let mut writefile = BufWriter::new(File::create("swish.csv")
        .expect("Failed to create swish.csv"));

    // write_to_csv(&mut writefile, "Name".to_string(), "Amount".to_string());

    while let Ok(line) = readfile.fill_buf() {
        if line.len() == 0 {
            break;
        }
        let line = String::from_utf8(line.to_vec()).unwrap();
        let mut split = line.split_whitespace();
        let name = split.next().unwrap().to_string();
        let amount = split.next().unwrap().to_string();
        write_to_csv(&mut writefile, name, amount);
        readfile.consume(line.len());
    }

    writefile.flush().unwrap();
}

/**
 * convert
 *  swish_ocr/swish1.jpg.txt:Firstname Lastname 12 kr
 * to
 *  Firstname Lastname 12
 */
fn get_data(raw_data: String) -> Vec<String> {
    let rd = raw_data.split(",");
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
    return data;
}

// fn get_line(r: &mut BufReader<File>) -> String {
//     let mut line = String::new();
//     r.read_line(&mut line).unwrap();
//     return line;
// }

fn write_to_csv(w: &mut BufWriter<File>, name: String, amount: String) {
    let line = format!("{},{}\n", name, amount);
    w.write(line.as_bytes()).unwrap();
}