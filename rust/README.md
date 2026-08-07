# Rust Number Conversion

This directory contains 3 versions of number conversion applications in Rust.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
cargo build --bin version1
```

### Usage
```bash
cargo run --bin version1 -- 10
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
cargo build --bin version2
```

### Usage
```bash
cargo run --bin version2 -- 10
```

## Version 3
Converts numbers to words in Spanish using native Rust libraries.

### Installation
```bash
cargo build --bin version3
```

### Usage
```bash
cargo run --bin version3 -- 10
```
