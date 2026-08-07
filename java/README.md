# Java Number Conversion

This directory contains 3 versions of number conversion applications in Java.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
mvn install
```

### Usage
```bash
mvn exec:java -Dexec.mainClass="Version1" -Dexec.args="10"
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
mvn install
```

### Usage
```bash
mvn exec:java -Dexec.mainClass="Version2" -Dexec.args="10"
```

## Version 3
Converts numbers to words in Spanish using native Java libraries.

### Installation
```bash
mvn install
```

### Usage
```bash
mvn exec:java -Dexec.mainClass="Version3" -Dexec.args="10"
```
