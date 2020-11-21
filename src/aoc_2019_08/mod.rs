use std::fs;
use crate::cli;

#[cfg(test)]
mod test;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_08.txt");
    let digits = fs::read_to_string(filename).unwrap();
    let img = parse(25, 6, digits.trim());
    println!("Part One: {}", part_one(&img));
}

fn part_one(img: &Image) -> usize {
    let zeroy_layer = img.layers()
        .fold((usize::max_value(), None), |(min, prev), l| {
            let n = l.count_of(0);
            if n < min {
                (n, Some(l))
            } else {
                (min, prev)
            }
        }).1.unwrap();
    zeroy_layer.count_of(1) * zeroy_layer.count_of(2)
}

#[derive(Debug)]
struct Image {
    width: u8,
    height: u8,
    digits: Vec<u8>,
}

impl Image {

    pub fn layer_count(&self) -> usize {
        self.digits.len() / self.layer_size()
    }

    pub fn get_layer(&self, i: usize) -> Layer {
        let layer_size = self.layer_size();
        Layer {
            width: self.width,
            height: self.height,
            data: &self.digits[(i * layer_size)..((i + 1) * layer_size)],
        }
    }

    pub fn layers(&self) -> Layers {
        Layers {
            image: self,
            curr: 0,
        }
    }

    #[inline]
    fn layer_size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

struct Layers<'a> {
    image: &'a Image,
    curr: usize,
}

impl<'a> Iterator for Layers<'a> {
    type Item = Layer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.image.layer_count() {
            return None
        }
        let i = self.curr;
        self.curr += 1;
        Some(self.image.get_layer(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let r = self.image.layer_count() - self.curr;
        (r, Some(r))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.curr += n;
        self.next()
    }
}

#[derive(Debug)]
struct Layer<'a> {
    width: u8,
    height: u8,
    data: &'a [u8],
}

impl Layer<'_> {

    pub fn count_of(&self, digit: u8) -> usize {
        self.data.iter().filter(|d| **d == digit).count()
    }

}

fn parse(width: u8, height: u8, data: &str) -> Image {
    println!("parse {}x{} w/ {} digits: {}...{}", width, height, data.len(), &data[0..5], &data[(data.len() - 5)..data.len()]);
    assert_eq!(0, data.len() % ((width * height) as usize));
    let mut digits: Vec<u8> = Vec::new();
    digits.reserve(data.len());
    for c in data.chars() {
        digits.push(parse_digit(c))
    }
    Image {
        width,
        height,
        digits,
    }
}

fn parse_digit(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("unrecognized character '{}'", c),
    }
}
