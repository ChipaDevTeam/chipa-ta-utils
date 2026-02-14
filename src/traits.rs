use core::fmt;

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
