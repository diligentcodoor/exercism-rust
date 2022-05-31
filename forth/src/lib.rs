use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    symbols: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_lowercase();
        for definition_or_statement in input.split_inclusive(";") {
            let definition_or_statement = definition_or_statement.trim();
            println! {"definition_or_statement: {}", definition_or_statement};
            println! {"stack: {:?}", self.stack};
            println! {"symbols: {:?}", self.symbols};
            if definition_or_statement.starts_with(":") && definition_or_statement.ends_with(";") {
                self.eval_definition(definition_or_statement)?;
            } else {
                match definition_or_statement.split(" : ").collect::<Vec<_>>()[..] {
                    [statement, definition] => {
                        self.eval_statement(statement)?;
                        let mut prefix = ": ".to_owned();
                        prefix.push_str(definition);
                        self.eval_definition(&prefix)?;
                    }
                    [statement] => self.eval_statement(statement)?,
                    _ => return Err(Error::InvalidWord),
                }
            }
        }
        Ok(())
    }

    fn eval_definition(&mut self, input: &str) -> Result {
        let words = input.split(" ").collect::<Vec<_>>();
        let symbol = words[1];
        if symbol.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let definition = words[2..words.len() - 1].join(" ");
        self.symbols.insert(symbol.to_string(), definition);
        Ok(())
    }

    // fn eval_definition(&mut self, input: &str) -> Result {
    //     let input = input.clone();
    //     let words = input.split(" ").collect::<Vec<_>>();
    //     let symbol = words[1];
    //     if symbol.parse::<Value>().is_ok() {
    //         return Err(Error::InvalidWord);
    //     }
    //     let definition = words[2..words.len() - 1].join(" ");
    //     let definition = self.substitute_symbols(definition);
    //     self.symbols.insert(symbol.to_string(), definition);
    //     Ok(())
    // }

    fn eval_statement(&mut self, input: &str) -> Result {
        let input = input.clone();
        let input = self.substitute_symbols(input.to_string());
        for word in input.split(" ") {
            match word {
                word if word.parse::<Value>().is_ok() => {
                    self.stack.push(word.parse::<Value>().unwrap());
                }
                "+" => self.add()?,
                "-" => self.subtract()?,
                "*" => self.multiply()?,
                "/" => self.divide()?,
                "dup" => self.dup()?,
                "drop" => self.drop()?,
                "swap" => self.swap()?,
                "over" => self.over()?,
                ":" => return Err(Error::InvalidWord),
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }

    fn resolve_symbol(&self, symbol: &str) -> String {
        println! {"resolve_symbol: {}", symbol};
        match self.symbols.get(symbol) {
            Some(definition) => self.resolve_symbol(definition),
            None => symbol.to_string(),
        }
    }

    fn substitute_symbols(&self, input: String) -> String {
        input
            .split(" ")
            .map(|word| self.resolve_symbol(word))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn dup(&mut self) -> Result {
        if let Some(top) = self.stack.pop() {
            self.stack.push(top);
            self.stack.push(top);
        } else {
            return Err(Error::StackUnderflow);
        }
        Ok(())
    }

    fn drop(&mut self) -> Result {
        if let Some(_) = self.stack.pop() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            self.stack.push(a);
            self.stack.push(b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            self.stack.push(b);
            self.stack.push(a);
            self.stack.push(b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn pop_2(&mut self) -> Option<(Value, Value)> {
        if self.stack.len() < 2 {
            return None;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        Some((a, b))
    }

    fn add(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            self.stack.push(a + b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn subtract(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            self.stack.push(b - a);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn multiply(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            self.stack.push(a * b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn divide(&mut self) -> Result {
        if let Some((a, b)) = self.pop_2() {
            if a == 0 {
                return Err(Error::DivisionByZero);
            }
            self.stack.push(b / a);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
