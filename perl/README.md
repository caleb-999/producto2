# Perl Number Conversion

This directory contains 3 versions of number conversion applications in Perl.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
cpan SOAP::Lite
```

### Usage
```bash
perl version1.pl 10
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
cpan SOAP::Lite
cpan Lingua::Translate
```

### Usage
```bash
perl version2.pl 10
```

## Version 3
Converts numbers to words in Spanish using native Perl libraries.

### Installation
```bash
cpan Lingua::ES::Numeros
```

### Usage
```bash
perl version3.pl 10
```
