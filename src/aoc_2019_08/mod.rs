use std::fs;
use crate::cli;

#[cfg(test)]
mod test;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_08.txt");
    let digits = fs::read_to_string(filename).unwrap();
    let img = parse(25, 6, digits.trim());
    println!("Part One: {}", part_one(&img));
    println!("Part Two:\n{}", part_two(&img));
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

fn part_two(img: &Image) -> String {
    // the text is drawn white-on-black, which is hard to read in ASCII Art
    let flat = img.flatten().invert();
    let layer = flat.get_layer(0);
    render_layer(&layer)
}

fn render_layer(layer: &Layer) -> String {
    let mut start_end = "-".repeat(layer.width + 2);
    start_end.insert(0, '+');
    start_end.push('+');
    start_end.push('\n');
    let mut s = String::from(&start_end);
    for r in 0..layer.height {
        s += "| ";
        for c in 0..layer.width {
            s.push(match layer.data[r * layer.width + c] {
                BLACK => '#',
                WHITE => ' ',
                TRANSPARENT => 'O',
                _ => '?',
            })
        }
        s += " |\n";
    }
    s += &start_end;
    s
}

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    digits: Vec<u8>,
}

const BLACK: u8 = 0;
const WHITE: u8 = 1;
const TRANSPARENT: u8 = 2;

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
        self.width * self.height
    }

    pub fn flatten(&self) -> Image {
        let digits = self.get_layer(0)
            .data
            .to_vec();
        let digits = self.layers()
            .skip(1)
            .fold(digits, |ds, l|
                ds.iter()
                    .zip(l.data)
                    .map(|(&bg, &d)|
                        if d != TRANSPARENT && bg == TRANSPARENT {
                            d
                        } else {
                            bg
                        }
                    )
                    .collect()
            );
        Image {
            width: self.width,
            height: self.height,
            digits,
        }
    }

    pub fn invert(&self) -> Image {
        let digits = self.digits
            .iter()
            .map(|&d| match d {
                BLACK => WHITE,
                WHITE => BLACK,
                _ => d,
            })
            .collect();
        Image {
            width: self.width,
            height: self.height,
            digits,
        }
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
    width: usize,
    height: usize,
    data: &'a [u8],
}

impl Layer<'_> {

    pub fn count_of(&self, digit: u8) -> usize {
        self.data.iter().filter(|&&d| d == digit).count()
    }

}

fn parse(width: usize, height: usize, data: &str) -> Image {
    println!("parse {}x{} w/ {} digits: {}...{}", width, height, data.len(), &data[0..5], &data[(data.len() - 5)..data.len()]);
    assert_eq!(0, data.len() % (width * height));
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
