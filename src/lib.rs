#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Generic debouncer with internal state.
#[derive(Debug, Default)]
pub struct Debouncer<T>
where
    T: Default,
{
    /// Processed value.
    value: T,

    /// Tick number of last value change.
    last_change: Option<u64>,
}

impl<T> Debouncer<T>
where
    T: Default + Copy + PartialEq,
{
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the debouncer with a new input value and returns the processed value.
    /// - `value` is the raw input value to be debounced.
    /// - `tick` is the current timestamp tick.
    /// - `freeze_ticks` is the number of ticks for that a value is freezed after a change.
    pub fn update(&mut self, value: T, tick: u64, freeze_ticks: u32) -> T {
        if value == self.value {
            return self.value;
        }

        match self.last_change {
            Some(last_change) => {
                if tick >= last_change + freeze_ticks as u64 {
                    self.value = value;
                    self.last_change = Some(tick);
                }
            }
            None => {
                // At first value change, use input value without any further checks.
                self.value = value;
                self.last_change = Some(tick);
            }
        }

        self.value
    }

    /// Returns the last processed value.
    pub fn value(&self) -> T {
        self.value
    }

    /// Sets a new value as processed value.
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    /// Returns the tick number of last value change or `None` if the value hasn't changed yet.
    pub fn last_change(&self) -> Option<u64> {
        self.last_change
    }
}

#[cfg(test)]
mod tests;
