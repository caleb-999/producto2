# C++ Number Conversion

This directory contains 3 versions of number conversion applications in C++.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libcurl4-openssl-dev libtinyxml2-dev

# Compile
g++ version1.cpp -lcurl -ltinyxml2 -o version1
```

### Usage
```bash
./version1 10
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install libcurl4-openssl-dev libtinyxml2-dev

# Compile
g++ version2.cpp -lcurl -ltinyxml2 -o version2
```

### Usage
```bash
./version2 10
```

## Version 3
Converts numbers to words in Spanish using native C++ libraries.

### Installation
```bash
# Compile
g++ version3.cpp -o version3
```

### Usage
```bash
./version3 10
```
