// Copyright (C) 2018 David A Holmes Jr
//
// This file is part of ToyCalc.
//
// ToyCalc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ToyCalc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ToyCalcCpp.  If not, see <http://www.gnu.org/licenses/>.

pub const MAX_DIGITS: u32 = 8;

#[derive(Copy,Clone)]
pub enum Operation {
    NoOp = 0,
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operation {
    pub fn to_str(&self) -> &str {
        match self {
            &Operation::NoOp => "",
            &Operation::Add => "+",
            &Operation::Subtract => "-",
            &Operation::Multiply => "*",
            &Operation::Divide => "/",
        }
    }
}

#[derive(Copy,Clone)]
enum Display {
    Input = 0,
    Accumulator
}

pub struct Calculator {
    display_listeners: Vec<Box<Fn(String)>>,
    accumulator: i64,
    input: i64,
    stored_operand: i64,
    current_operation: Operation,
    stored_operation: Operation,
    current_display: Display,
    equals_pressed: bool,
    error: bool,
    error_string: String
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display_listeners: vec!(),
            accumulator: 0,
            input: 0,
            stored_operand: 0,
            current_operation: Operation::NoOp,
            stored_operation: Operation::NoOp,
            current_display: Display::Input,
            equals_pressed: false,
            error: false,
            error_string: String::new()
        }
    }

    pub fn press_digit(&mut self, digit: i8) {
        if self.equals_pressed || self.error {
            self.reset();
        };

        match self.current_display {
            Display::Accumulator => self.current_display = Display::Input,
            _ => (),
        };

        if self.input == 0 {
            self.input = digit as i64;
        } else if self.input < 10i64.pow(MAX_DIGITS-1) {
            self.input = self.input * 10i64 + digit as i64;
        };

        self.update_display();
    }

    pub fn press_operation(&mut self, operation: Operation) {
        if !self.equals_pressed {
            self.perform_operation();
            self.input = 0;
        }

        self.current_display = Display::Accumulator;
        self.current_operation = operation;
        self.equals_pressed = false;

        self.update_display();
    }

    pub fn press_equals(&mut self) {
        if self.equals_pressed {
            self.input = self.stored_operand;
            self.current_operation = self.stored_operation;
        }

        self.perform_operation();
        self.stored_operation = self.current_operation;
        self.current_operation = Operation::NoOp;
        self.stored_operand = self.input;
        self.equals_pressed = true;
        self.current_display = Display::Accumulator;
        self.input = 0;
        self.update_display();
    }

    pub fn perform_operation(&mut self) {
        match self.current_operation {
            Operation::NoOp => self.accumulator = self.input,
            Operation::Add => self.accumulator += self.input,
            Operation::Subtract => self.accumulator -= self.input,
            Operation::Multiply => self.accumulator *= self.input,
            Operation::Divide => if self.input == 0 {
                self.error = true;
                self.error_string = "error".to_string();
            } else {
                self.accumulator /= self.input;
            }
        }

        if self.accumulator >= 10i64.pow(MAX_DIGITS) {
            self.error = true;
            self.error_string = "overflow".to_string();
        }
    }

    pub fn add_display_listener(&mut self, listener: Box<Fn(String)>) {
        self.display_listeners.push(listener);
    }

    pub fn get_display_string(&self) -> String {
        if self.error {
            return self.error_string.clone()
        } else {
            return self.get_display_value().to_string()
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.input = 0;
        self.stored_operand = 0;
        self.current_operation = Operation::NoOp;
        self.stored_operation = Operation::NoOp;
        self.current_display = Display::Input;
        self.equals_pressed = false;
        self.error = false;
    }

    fn get_display_value(&self) -> i64 {
        match self.current_display {
            Display::Accumulator => self.accumulator,
            Display::Input => self.input,
        }
    }

    fn update_display(&self) {
        for listener in &self.display_listeners {
            (*listener)(self.get_display_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_press_digit() {
        let mut calc = Calculator::new();
        calc.press_digit(0);
        calc.press_digit(0);
        calc.press_digit(1);
        calc.press_digit(5);
        calc.press_digit(3);
        calc.press_digit(8);
        calc.press_digit(9);
        calc.press_digit(2);
        calc.press_digit(7);
        calc.press_digit(4);
        assert_eq!("15389274", calc.get_display_string());
    }

    #[test]
    fn test_press_digit_overflow() {
        let mut calc = Calculator::new();
        let mut expected_long = 0i64;

        for i in 1..MAX_DIGITS {
            expected_long = expected_long * 10i64 + (i as i64)
        }

        let expected_string = expected_long.to_string();

        for i in 1..MAX_DIGITS {
            calc.press_digit((i % 10) as i8)
        }

        assert_eq!(expected_string, calc.get_display_string());
    }

    #[test]
    fn test_operations() {
        let mut calc = Calculator::new();

        calc.press_digit(5);
        assert_eq!("5", calc.get_display_string());

        calc.press_operation(Operation::Add);
        assert_eq!("5", calc.get_display_string());

        calc.press_digit(6);
        assert_eq!("6", calc.get_display_string());

        calc.press_operation(Operation::Subtract);
        assert_eq!("11", calc.get_display_string());

        calc.press_digit(3);
        assert_eq!("3", calc.get_display_string());

        calc.press_equals();
        assert_eq!("8", calc.get_display_string());

        calc.press_operation(Operation::Multiply);
        assert_eq!("8", calc.get_display_string());

        calc.press_digit(3);
        assert_eq!("3", calc.get_display_string());

        calc.press_operation(Operation::Divide);
        assert_eq!("24", calc.get_display_string());

        calc.press_digit(2);
        assert_eq!("2", calc.get_display_string());

        calc.press_equals();
        assert_eq!("12", calc.get_display_string());

        calc.press_equals();
        assert_eq!("6", calc.get_display_string());

        calc.press_equals();
        assert_eq!("3", calc.get_display_string());
    }

    #[test]
    fn test_get_display_string() {
        let mut calc = Calculator::new();
        calc.press_digit(2);
        calc.press_digit(3);
        calc.press_digit(4);
        assert_eq!("234", calc.get_display_string());
        calc.press_operation(Operation::Add);
        assert_eq!("234", calc.get_display_string());
        calc.press_digit(5);
        calc.press_digit(6);
        calc.press_digit(7);
        assert_eq!("567", calc.get_display_string());
    }
}
