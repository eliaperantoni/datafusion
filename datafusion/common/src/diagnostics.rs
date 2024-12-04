use std::ops::{Deref, Range};

use ariadne::Report;

#[derive(Debug)]
pub struct Diagnostics(pub Report<'static, Range<usize>>);

