use crate::{eval::{Evaluator, Value}, Result, error, Error};

impl Evaluator {
    pub fn add(&mut self, line: usize, column: usize) -> Result<()> {
        let rhs = self.pop(line, column)?;
        let lhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs + rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs + rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn sub(&mut self, line: usize, column: usize) -> Result<()> {
        let rhs = self.pop(line, column)?;
        let lhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs - rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs - rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn mul(&mut self, line: usize, column: usize) -> Result<()> {
        let rhs = self.pop(line, column)?;
        let lhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs * rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs * rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn div(&mut self, line: usize, column: usize) -> Result<()> {
        let rhs = self.pop(line, column)?;
        let lhs = self.pop(line, column)?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => self.push(Value::Integer(lhs / rhs)),
                _ => error!(line, column, "Expected an Integer, found a {}.", rhs.get_type()),
            }
            Value::Real(lhs) => match rhs {
                Value::Real(rhs) => self.push(Value::Real(lhs / rhs)),
                _ => error!(line, column, "Expected a Real, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a Real or an Integer, found a {}.", lhs.get_type()),
        }
    }
    pub fn cat(&mut self, line: usize, column: usize) -> Result<()> {
        let rhs = self.pop(line, column)?;
        let lhs = self.pop(line, column)?;

        match lhs {
            Value::String(lhs) => match rhs {
                Value::String(rhs) => self.push(Value::String(format!("{}{}", lhs, rhs))),
                _ => error!(line, column, "Expected a String, found a {}.", rhs.get_type()),
            }
            _ => error!(line, column, "Expected a String, found a {}.", lhs.get_type()),
        }
    }

    pub fn app(&mut self, line: usize, column: usize) -> Result<()> {
        let val = self.pop(line, column)?;

        match val {
            Value::Quote(exprs) => exprs.into_iter().map(|expr| self.eval_expr(expr)).collect::<Result<()>>(),
            _ => error!(line, column, "Expected a Quote, found a {}.", val.get_type()),
        }
    }
    pub fn dup(&mut self, line: usize, column: usize) -> Result<()> {
        let to_dup = self.pop(line, column)?;
        self.push(to_dup.clone())?;
        self.push(to_dup)
    }
    pub fn pop_stack(&mut self, line: usize, column: usize) -> Result<()> {
        self.pop(line, column)?;
        Ok(())
    }
    pub fn swap(&mut self, line: usize, column: usize) -> Result<()> {
        let a = self.pop(line, column)?;
        let b = self.pop(line, column)?;
        self.push(a)?;
        self.push(b)
    }
    pub fn print_stack(&mut self, _: usize, _: usize) -> Result<()> {
        println!("{}", self.stack.clone().into_iter().map(|v| v.get_lit(true)).collect::<Vec<String>>().join(" "));
        Ok(())
    }
}
