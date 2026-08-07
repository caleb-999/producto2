# Golang Number Conversion

This directory contains 3 versions of number conversion applications in Go.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
go mod download
```

### Usage
```bash
go run version1.go 10
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
go mod download
```

### Usage
```bash
go run version2.go 10
```

## Version 3
Converts numbers to words in Spanish using native Go libraries.

### Installation
```bash
go mod download
```

### Usage
```bash
go run version3.go 10
```
