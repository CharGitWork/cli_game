use console::StyledObject;
use console;

use std::error;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::marker::{Send, Sync};

use crate::functionalities::structs::DataSet;

#[derive(Debug)]
pub struct ErrorView {
    error: StyledObject<String>,
    error_context: StyledObject<String>,
    act_context: AtomicBool,
}

impl ErrorView {
    pub fn new_neutral_err(error: String) -> ErrorView {
        ErrorView { 
            error: console::style(error),
            error_context: console::style(String::from("")),
            act_context: AtomicBool::new(false),
        }
    }

    pub fn new_style_err(error: StyledObject<String>) -> ErrorView {
        ErrorView { 
            error,
            error_context: console::style(String::from("")),
            act_context: AtomicBool::new(false),
        }
    }

    pub fn act_context(&mut self) {
        self.act_context.store(true, Ordering::SeqCst);
    }

    pub fn del_context(&mut self) {
        self.act_context.store(false, Ordering::SeqCst);
    }

    pub fn get_act_context(&self) -> bool {
        self.act_context.load(Ordering::SeqCst)
    }

    pub fn add_neutral_context(&mut self, context: String) {
        self.error_context = console::style(context);
    }

    pub fn add_style_context(&mut self, context: String) {
        self.error_context = console::style(context);
    }

    pub fn error(&self) -> &StyledObject<String> {
        &self.error
    }

    pub fn error_context(&self) -> &StyledObject<String> {
        &self.error_context
    }
}

impl fmt::Display for ErrorView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.get_act_context() {
            write!(f, "\ncontext: {}\n\n    Caused by: {}\n", self.error_context(), self.error())
        } else {
            write!(f, "\n   Caused by: {}\n", self.error())
        }
    }
}

pub fn out_limit(ds: &DataSet) -> ErrorView {
    let y = console::style(format!("{}", ds.posy())).bold().red();
    let x = console::style(format!("{}", ds.posx())).bold().red();
    let stl_obj = console::style(format!("Position out of the limit: Y:{} X:{}", y, x)).bold();
    ErrorView::new_style_err(stl_obj)
}

// fn verify<C>(cmp_1: &C, cmp_2: &C) -> bool
// where C: PartialEq + PartialOrd,
// {

// }

impl error::Error for ErrorView {}

unsafe impl Send for ErrorView {}
unsafe impl Sync for ErrorView {}