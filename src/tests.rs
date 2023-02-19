//! Unit tests

use super::*;

/// Single pulse boolean value.
#[test]
fn single_pulse_bool() {
    let mut debouncer = Debouncer::new();

    const DEBOUNCE_TICKS: u32 = 3;

    let input_values = [true, false, false, false, false, false, false, false];
    let expected_values = [true, true, true, false, false, false, false, false];

    for (tick, (input_value, expected_value)) in input_values
        .into_iter()
        .zip(expected_values.into_iter())
        .enumerate()
    {
        let value = debouncer.update(input_value, tick as u64, DEBOUNCE_TICKS);
        assert_eq!(value, expected_value);
    }
}

/// Single pulse u8 value.
#[test]
fn single_pulse_u8() {
    let mut debouncer = Debouncer::new();

    const DEBOUNCE_TICKS: u32 = 5;

    let input_values = [6, 0, 0, 0, 0, 0, 0, 0];
    let expected_values = [6, 6, 6, 6, 6, 0, 0, 0];

    for (tick, (input_value, expected_value)) in input_values
        .into_iter()
        .zip(expected_values.into_iter())
        .enumerate()
    {
        let value = debouncer.update(input_value, tick as u64, DEBOUNCE_TICKS);
        assert_eq!(value, expected_value);
    }
}

/// Single pulse f32 value.
#[test]
fn single_pulse_f32() {
    let mut debouncer = Debouncer::new();

    const DEBOUNCE_TICKS: u32 = 4;

    let input_values = [5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let expected_values = [5.0, 5.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0];

    for (tick, (input_value, expected_value)) in input_values
        .into_iter()
        .zip(expected_values.into_iter())
        .enumerate()
    {
        let value = debouncer.update(input_value, tick as u64, DEBOUNCE_TICKS);
        assert_eq!(value, expected_value);
    }
}

/// Single pulse u8 value with initial delay.
#[test]
fn delayed_pulse_u8() {
    let mut debouncer = Debouncer::new();

    const DEBOUNCE_TICKS: u32 = 4;

    let input_values = [0, 0, 6, 0, 0, 0, 0, 0];
    let expected_values = [0, 0, 6, 6, 6, 6, 0, 0];

    for (tick, (input_value, expected_value)) in input_values
        .into_iter()
        .zip(expected_values.into_iter())
        .enumerate()
    {
        let value = debouncer.update(input_value, tick as u64, DEBOUNCE_TICKS);
        assert_eq!(value, expected_value);
    }
}

/// Bouncing u8 value.
#[test]
fn bouncing_u8() {
    let mut debouncer = Debouncer::new();

    const DEBOUNCE_TICKS: u32 = 5;

    let input_values = [2, 4, 3, 0, 7, 5, 6, 4];
    let expected_values = [2, 2, 2, 2, 2, 5, 5, 5];

    for (tick, (input_value, expected_value)) in input_values
        .into_iter()
        .zip(expected_values.into_iter())
        .enumerate()
    {
        let value = debouncer.update(input_value, tick as u64, DEBOUNCE_TICKS);
        assert_eq!(value, expected_value);
    }
}
