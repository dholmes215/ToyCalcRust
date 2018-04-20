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

#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

extern crate toycalc;

use toycalc::*;

use gtk::prelude::*;

use relm::Widget;
use relm_attributes::widget;

trait CalcButton {
    fn set_calcbutton(&self, calcbutton: bool);
    fn set_biglabel(&self, markup: &str);
}

impl CalcButton for gtk::Button {
    fn set_calcbutton(&self, calcbutton: bool) {
        if calcbutton {
            // GTK lays out widgets according to their expand and size request
            // properties. I'm overriding the defaults here to cause them to expand
            // and to prevent them from being too small.
            self.set_hexpand(true);
            self.set_vexpand(true);
            self.set_size_request(50, 50);
        }
    }

    fn set_biglabel(&self, markup: &str) {
        self.set_label(""); // Create the child label widget so we can get it
        match self.get_child() {
            Some(ref child) => match child.clone().downcast::<gtk::Label>() {
                Ok(ref button_label) =>
                    button_label.set_markup(&*format!("<span font='24'>{}</span>", markup)),
                Err(_) => panic!("gtk::Button child wasn't a gtk::Label (should never happen)"),
            },
            None => panic!("gtk::Button didn't have a child (should never happen)"),
        }
    }
}

pub struct Model {
    calc: Calculator,
    display_markup: String
}

#[derive(Msg)]
pub enum Msg {
    Digit(i8),
    Operation(Operation),
    Equals,
    Quit,
}

#[widget]
impl Widget for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model() -> Model {
        Model {
            calc: Calculator::new(),
            display_markup: "<span font='32'>0</span>".to_string()
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Digit(d) => self.model.calc.press_digit(d),
            Msg::Operation(o) => self.model.calc.press_operation(o),
            Msg::Equals => self.model.calc.press_equals(),
            Msg::Quit => gtk::main_quit(),
        };
        // Automatically causes view to update.
        self.model.display_markup = format!("<span font='32'>{}</span>", self.model.calc.get_display_string())
    }

    view! {
        gtk::Window {
            title: "Relm Calculator",
            border_width: 10,
            position: gtk::WindowPosition::Center,
            delete_event(_,_) => (Msg::Quit, Inhibit(false)),
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 10,
                gtk::Label {
                    halign: gtk::Align::End,
                    // Automatically updates when model changes.
                    markup: &self.model.display_markup.to_string(),
                },
                gtk::Grid {
                    gtk::Button { biglabel: "7", calcbutton: true, cell: { left_attach: 0, top_attach: 0 },
                        clicked => Msg::Digit(7) },
                    gtk::Button { biglabel: "8", calcbutton: true, cell: { left_attach: 1, top_attach: 0 },
                        clicked => Msg::Digit(8) },
                    gtk::Button { biglabel: "9", calcbutton: true, cell: { left_attach: 2, top_attach: 0 },
                        clicked => Msg::Digit(9) },
                    gtk::Button { biglabel: "+", calcbutton: true, cell: { left_attach: 3, top_attach: 0 },
                        clicked => Msg::Operation(Operation::Add) },
                    gtk::Button { biglabel: "4", calcbutton: true, cell: { left_attach: 0, top_attach: 1 },
                        clicked => Msg::Digit(4) },
                    gtk::Button { biglabel: "5", calcbutton: true, cell: { left_attach: 1, top_attach: 1 },
                        clicked => Msg::Digit(5) },
                    gtk::Button { biglabel: "6", calcbutton: true, cell: { left_attach: 2, top_attach: 1 },
                        clicked => Msg::Digit(6) },
                    gtk::Button { biglabel: "-", calcbutton: true, cell: { left_attach: 3, top_attach: 1 },
                        clicked => Msg::Operation(Operation::Subtract) },
                    gtk::Button { biglabel: "1", calcbutton: true, cell: { left_attach: 0, top_attach: 2 },
                        clicked => Msg::Digit(1) },
                    gtk::Button { biglabel: "2", calcbutton: true, cell: { left_attach: 1, top_attach: 2 },
                        clicked => Msg::Digit(2) },
                    gtk::Button { biglabel: "3", calcbutton: true, cell: { left_attach: 2, top_attach: 2 },
                        clicked => Msg::Digit(3) },
                    gtk::Button { biglabel: "*", calcbutton: true, cell: { left_attach: 3, top_attach: 2 },
                        clicked => Msg::Operation(Operation::Multiply) },
                    gtk::Button { biglabel: "0", calcbutton: true, cell: { left_attach: 0, top_attach: 3, width: 2 },
                        clicked => Msg::Digit(0) },
                    gtk::Button { biglabel: "=", calcbutton: true, cell: { left_attach: 2, top_attach: 3 },
                        clicked => Msg::Equals },
                    gtk::Button { biglabel: "/", calcbutton: true, cell: { left_attach: 3, top_attach: 3 },
                        clicked => Msg::Operation(Operation::Divide) },
                },
            }
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
