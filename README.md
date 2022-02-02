# Sloggrs

Simple logging helper written in rust

## Log levels

- DEBUG: Debug information, (not needed in production)
- INFO: Informations about progress (OK for production)
- WARN: Warnings, nothing really dangerous but may be useful to find potential errors
- ERROR: Recoverable errors
- FATAL: Unrecoverable errors

## Example

```rust
#[macro_use]
extern crate sloggrs;

fn main() {
    // Log only warnings and superior (enables WARN > ERROR > FATAL logs levels)
    init!(WARN);

    // Will **not** be logged
    debug!("Some really interesting debug message");

    // Will **not** be logged
    info!("Random information");

    // Will be logged
    warn!("Some information");

    // Will be logged
    error!("Failed to fail!")

    // Will be logged
    fatal!("Picards!");
}
```
