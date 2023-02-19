# timed-debouncer

`no_std` Rust crate for debouncing signals from electrical contacts like push-buttons, rotary switches and alike. The processing uses a time-based generic approach and can therefore be used with any simple numeric type (e.g. `bool`, `u8` or `f32`).

## Usage

Call the `update()` function with a raw input value, a timestamp tick and a tick count for the debouncing.
It will return a processed value that is stable for the configured tick count.
The tick value can be of any unit, in practice, using milliseconds is a common recommendation. The required debounce time is dependent on the mechanics of the contacts. Using a value in range of 10-20ms is a good choice for most switches to ensure safe operation.

The debouncer will take the default value of the used type as initial value, which is typically `0` for integer types and `false` for `bool`. You can use the `set_value()` method to set a different initial value.

## Example

```rust
/// Create an instance of the debouncer.
let mut debouncer = timed_debouncer::Debouncer::new();

/// Number of ticks used to freeze the value when processing.
const DEBOUNCE_TICKS: u32 = 20;

// In real world, this is done somewhere in the main loop.
for tick in 0..50 {
    // Provide real values read from pins here.
    let input_value = if tick == 0 { 1 } else { 0 };

    let debounced_value = debouncer.update(input_value, tick, DEBOUNCE_TICKS);
    println!("{}: {:?} {:?}", tick, input_value, debounced_value);
}

```

## Tests

Run `cargo test` for the unit tests.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
