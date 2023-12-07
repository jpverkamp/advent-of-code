#[derive(Debug)]
pub struct Number {
    pub value: usize,
    pub x_min: usize,
    pub x_max: usize,
    pub y: usize,
}

impl Number {
    pub fn is_neighbor(&self, x: usize, y: usize) -> bool {
        x + 1 >= self.x_min && x <= self.x_max && y + 1 >= self.y && y <= self.y + 1
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub value: char,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

impl From<String> for Schematic {
    fn from(value: String) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        fn finish_number(
            numbers: &mut Vec<Number>,
            digits: &mut String,
            x_min: usize,
            x_max: usize,
            y: usize,
        ) {
            if digits.is_empty() {
                return;
            }

            let value = digits.parse::<usize>().unwrap();
            digits.clear();
            numbers.push(Number {
                value,
                x_min,
                x_max,
                y,
            });
        }

        for (y, line) in value.lines().enumerate() {
            let mut digits = String::new();
            let mut x_min = 0;

            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if digits.is_empty() {
                        x_min = x;
                    }
                    digits.push(c);
                } else if c == '.' {
                    finish_number(&mut numbers, &mut digits, x_min, x, y);
                } else {
                    finish_number(&mut numbers, &mut digits, x_min, x, y);
                    symbols.push(Symbol { value: c, x, y });
                }
            }
            finish_number(&mut numbers, &mut digits, x_min, line.len(), y);
        }

        Schematic { numbers, symbols }
    }
}