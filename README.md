# Wifi on ESP32C3 and ESP32 (on bare-metal Rust)

## About

This is experimental and work-in-progress! You are welcome to experiment with it and contribute but probably shouldn't use this for something real yet.

This uses the WiFi driver found in https://github.com/espressif/esp-wireless-drivers-3rdparty

## Version used

esp-wireless-drivers-3rdparty-055f1ef49d0cb72c24bd492fbbdd37497a90bdae
45701c0

https://github.com/espressif/esp-wireless-drivers-3rdparty/archive/45701c0.zip

## Example

- dhcp
    - set SSID and PASSWORD env variable
    - gets an ip address via DHCP
    - performs an HTTP get request to some "random" server

|Command|Chip|
|---|---|
|`cargo "+nightly" run --example dhcp_esp32c3 --release --target riscv32i-unknown-none-elf --features "esp32c3,embedded-svc"`|ESP32C3|
|`cargo "+esp" run --example dhcp_esp32 --release --target xtensa-esp32-none-elf --features "esp32,embedded-svc"`|ESP32|

Additional you can specify these features
|Feature|Meaning|
|---|---|
|wifi_logs|logs the WiFi logs from the driver at log level info|
|dump_packets|dumps some packet info at log level info|
|utils|Provide utilities to use `embedded-nal`, this is a default feature|
|allocator|Provides a global allocator which just uses the internal `malloc` - it doesn't honor special alignments|
|embedded-svc|Provides a (very limited) implementation of the `embedded-svc` WiFi trait, includes `utils` and `allocator` feature|

In general you should use the release profile since otherwise the performance is quite bad.

## What works?

- scanning for WiFi access points
- connect to WiFi access point

## Notes on ESP32C3 support

- uses SYSTIMER as the main timer
- doesn't work in direct-boot mode
- only works with target `riscv32i-unknown-none-elf`

## Notes on ESP32 support

This is even more experimental than support for ESP32C3.

- The WiFi logs only print the format string - not the actual values. 
- Also there might be some packet loss and a bit worse performance than on ESP32C3 currently. 
- The code runs on a single core and is currently not multi-core safe!

On ESP32 currently TIMG1 is used as the main timer so you can't use it for anything else.

## Directory Structure

- `src/timer-espXXX.rs`: systimer code used for timing and task switching
- `src/preemt/`: a bare minimum RISCV and Xtensa round-robin task scheduler
- `src/log/`: code used for logging
- `src/binary/`: generated bindings to the WiFi driver (per chip)
- `src/compat/`: code needed to emulate enough of an (RT)OS to use the driver
  - `malloc.rs`: a homegrown allocator - this is NOT used on the Rust side (the Rust side of this is currently no-alloc)
  - `common.rs`: basics like semaphores and recursive mutexes
  - `timer_compat.rs`: code to emulate timer related functionality
- `headers`: headers found in the WiFi driver archive (bindings are generated from these)
- `libs/espXXX`: static libraries found in the WiFi driver archive (these get linked into the binary)
- `mkbindings.bat`: generate the bindings / just calls `bindgen`
- `ld/espXXX/rom_functions.x`: the WiFi driver uses some of these so it needs to get linked
- `ld/esp32/wifi-link.x`: the main linker script for ESP32 - needs to get cleaned up and ideally the changes move to ESP-HAL
- `examples/dhcp_espXXX/main.rs`: example using the code (per chip)

## Missing / To be done

- lots of refactoring
- Bluetooth (and coex)
- esp-now
- powersafe support

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
