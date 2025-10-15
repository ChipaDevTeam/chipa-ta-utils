use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{Candle, Period, Reset, TaUtilsError, TaUtilsResult};

#[cfg(feature = "schemas")]
use schemars::JsonSchema;

#[cfg_attr(feature = "schemas", derive(JsonSchema))]
#[derive(Debug, PartialEq, Clone)]
pub struct Bar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub price: f64,
    pub volume: f64,
}

/// Market data passed to strategies and indicators.
/// Contains OHLCV values.
#[cfg_attr(feature = "schemas", derive(JsonSchema))]
#[derive(Debug, Clone)]
pub enum MarketData {
    Bar(Bar), // Boxed trait object for dynamic dispatch
    // Add more variants as needed for other Candle implementors
    Float(f64),
}

// Can you help me emprove the Queue struct? the goal is to make it like a Vec but with a fixed capacity that removes the oldest element when a new one is added beyond its capacity.
// it also implements the Period and Reset traits, allowing it to be used in a similar way to Cycle.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Queue<T> {
    queue: Vec<T>,
    period: usize,
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            open: 0.0,
            close: 0.0,
            low: 0.0,
            high: 0.0,
            price: 0.0,
            volume: 0.0,
        }
    }
}

impl Bar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_open<T: Into<f64>>(mut self, val: T) -> Self {
        self.open = val.into();
        self
    }

    pub fn set_high<T: Into<f64>>(mut self, val: T) -> Self {
        self.high = val.into();
        self
    }

    pub fn set_low<T: Into<f64>>(mut self, val: T) -> Self {
        self.low = val.into();
        self
    }

    pub fn set_close<T: Into<f64>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }

    pub fn set_price<T: Into<f64>>(mut self, val: T) -> Self {
        self.price = val.into();
        self
    }

    pub fn set_volume(mut self, val: f64) -> Self {
        self.volume = val;
        self
    }

    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
}

impl Candle for Bar {
    fn close(&self) -> f64 {
        self.close
    }

    fn open(&self) -> f64 {
        self.open
    }

    fn high(&self) -> f64 {
        self.high
    }

    fn low(&self) -> f64 {
        self.low
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn volume(&self) -> f64 {
        self.volume
    }
}



impl MarketData {
    /// Returns the typical price ((high + low + close) / 3).
    pub fn typical_price(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.typical_price(),
            MarketData::Float(value) => *value,
        }
    }
}

impl Candle for MarketData {
    fn open(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.open(),
            MarketData::Float(value) => *value,
        }
    }

    fn close(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.close(),
            MarketData::Float(value) => *value,
        }
    }

    fn high(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.high(),
            MarketData::Float(value) => *value,
        }
    }

    fn low(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.low(),
            MarketData::Float(value) => *value,
        }
    }

    fn price(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.price(),
            MarketData::Float(value) => *value,
        }
    }

    fn volume(&self) -> f64 {
        match self {
            MarketData::Bar(bar) => bar.volume(),
            MarketData::Float(_) => f64::NAN, // Volume not applicable for Float variant
        }
    }
}

impl<T: Default + Clone> Queue<T> {
    pub fn new(period: usize) -> TaUtilsResult<Self> {
        if period == 0 {
            return Err(TaUtilsError::InvalidParameter(
                "Period must be greater than 0".to_string(),
            ));
        }
        Ok(Self {
            queue: Vec::with_capacity(period),
            period,
        })
    }

    pub fn push(&mut self, value: T) -> Option<T> {
        self.queue.push(value);
        if self.queue.len() > self.period {
            let removed = self.queue.remove(0);
            Some(removed)
        } else {
            None
        }
    }
}

impl<T> Deref for Queue<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<T> DerefMut for Queue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl<T> Period for Queue<T> {
    fn period(&self) -> usize {
        self.period
    }
}
impl<T> Reset for Queue<T> {
    fn reset(&mut self) {
        self.queue = Vec::with_capacity(self.period);
    }
}
