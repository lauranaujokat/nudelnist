use std::io::{BufRead, Read, Write};

#[derive(PartialEq)]
pub struct Img {
    pub number: u8,
    pub data: Vec<f64>,
}

pub fn images_from_csv(filename: &str) -> Vec<Img> {
    let file = std::fs::File::open(filename).unwrap();
    let data = std::io::BufReader::new(file);
    let lines = data.lines().skip(1).flatten();

    lines
        .map(|line| {
            let mut nums = line.split(',');
            Img {
                number: nums.next().unwrap().parse().unwrap(),
                data: nums
                    .map(|num| num.parse::<u8>().unwrap() as f64 / 256.)
                    .collect(),
            }
        })
        .collect()
}
pub fn images_from_bin(filename: &str) -> Vec<Img> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();

    data.chunks(28 * 28 + 1)
        .map(|chunk| {
            let mut nums = chunk.iter();
            Img {
                number: *nums.next().unwrap(),
                data: nums.map(|num| *num as f64 / 256.).collect(),
            }
        })
        .collect()
}
pub fn images_from_xz(filename: &str) -> Vec<Img> {
    let file = std::fs::File::open(filename).unwrap();
    let mut encoder = xz::read::XzDecoder::new(file);

    let mut data = vec![];
    encoder.read_to_end(&mut data).unwrap();

    data.chunks(28 * 28 + 1)
        .map(|chunk| {
            let mut nums = chunk.iter();
            Img {
                number: *nums.next().unwrap(),
                data: nums.map(|num| *num as f64 / 256.).collect(),
            }
        })
        .collect()
}

pub fn csv_to_bin(csv_file: &str, bin_file: &str) {
    let in_file = std::fs::File::open(csv_file).unwrap();
    let reder = std::io::BufReader::new(in_file);

    let out_file = std::fs::File::create(bin_file).unwrap();
    let mut writer = std::io::BufWriter::new(out_file);

    let lines = reder.lines().skip(1).flatten();
    for line in lines {
        for num in line.split(',') {
            let num = num.parse::<u8>().unwrap();
            writer.write_all(std::array::from_ref(&num)).unwrap();
        }
    }
}
pub fn csv_to_xz(csv_file: &str, xz_file: &str) {
    let in_file = std::fs::File::open(csv_file).unwrap();
    let reder = std::io::BufReader::new(in_file);

    let out_file = std::fs::File::create(xz_file).unwrap();
    let mut writer = xz::write::XzEncoder::new(out_file, 9);

    let lines = reder.lines().skip(1).flatten();
    for line in lines {
        for num in line.split(',') {
            let num = num.parse::<u8>().unwrap();
            writer.write_all(std::array::from_ref(&num)).unwrap();
        }
    }
}
