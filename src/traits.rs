use core::fmt;

use chipa_types::Number;

use crate::{Bar, OutputShape, TaUtilsError, errors::TaUtilsResult};

pub trait Candle: fmt::Debug {
    fn open(&self) -> f64 {
        self.price()
    }

    fn close(&self) -> f64 {
        self.price()
    }

    fn high(&self) -> f64 {
        self.price()
    }

    fn low(&self) -> f64 {
        self.price()
    }

    fn price(&self) -> f64;

    fn volume(&self) -> f64 {
        f64::NAN
    }

    fn to_bar(&self) -> Bar {
        Bar::new()
            .set_open(self.open())
            .set_high(self.high())
            .set_low(self.low())
            .set_close(self.close())
            .set_price(self.price())
            .set_volume(self.volume())
    }
}

pub trait Next<T> {
    type Output;

    fn next(&mut self, input: T) -> TaUtilsResult<Self::Output>;
}

impl Next<&dyn Candle> for () {
    type Output = f64;

    fn next(&mut self, _: &dyn Candle) -> TaUtilsResult<Self::Output> {
        Err(TaUtilsError::InvalidParameter(
            "Cannot call next on unit type".to_string(),
        ))
    }
}

pub trait IndicatorTrait: fmt::Debug + fmt::Display + Reset + Period {
    fn output_shape(&self) -> OutputShape;

    fn name(&self) -> String {
        self.to_string()
    }
}

/// Resets an indicator to the initial state.
pub trait Reset {
    fn reset(&mut self);
}

pub trait Period {
    fn period(&self) -> usize;
}


// Implement Reset for default types
impl Reset for () {
    fn reset(&mut self) {
        // No state to reset
    }
}

impl<T> Reset for Vec<T> {
    fn reset(&mut self) {
        self.clear();
    }    
}

impl Reset for f64 {
    fn reset(&mut self) {
    }
}

impl Reset for Number {
    fn reset(&mut self) {
    }    
}

impl<T> Reset for Option<T> {
    fn reset(&mut self) {
        *self = None;
    }
}

impl Reset for usize {
    fn reset(&mut self) {
    }
} 

impl Reset for bool {
    fn reset(&mut self) {
        *self = false;
    }
}

impl Reset for String {
    fn reset(&mut self) {
        self.clear();
    }
}

impl Period for () {
    fn period(&self) -> usize {
        0
    }
}

impl Period for f64 {
    fn period(&self) -> usize {
        0
    }
}

impl Period for Number {
    fn period(&self) -> usize {
        0
    }
}

impl Period for usize {
    fn period(&self) -> usize {
        0
    }
}

impl Period for bool {
    fn period(&self) -> usize {
        0
    }
}

impl Period for String {
    fn period(&self) -> usize {
        0
    }
}

impl<T> Period for Vec<T> {
    fn period(&self) -> usize {
        self.len()
    }
}


impl<T: Period> Period for Option<T> {
    fn period(&self) -> usize {
        match self {
            Some(t) => t.period(),
            None => 0,
        }
    }
}

impl<T> Period for Box<[T]> {
    fn period(&self) -> usize {
        self.len()
    }
}

