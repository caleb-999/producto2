# .NET 10 Number Conversion

This directory contains 3 versions of number conversion applications in .NET 10.

## Version 1
Consumes the SOAP service directly to convert numbers to words.

### Installation
```bash
dotnet restore Version1.csproj
```

### Usage
```bash
dotnet run --project Version1.csproj -- 10
```

## Version 2
Consumes the SOAP service and translates the result from English to Spanish.

### Installation
```bash
dotnet restore Version2.csproj
```

### Usage
```bash
dotnet run --project Version2.csproj -- 10
```

## Version 3
Converts numbers to words in Spanish using native .NET libraries.

### Installation
```bash
dotnet restore Version3.csproj
```

### Usage
```bash
dotnet run --project Version3.csproj -- 10
```
