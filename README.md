# Conversión de Números a Letras en Múltiples Lenguajes de Programación

Este proyecto contiene implementaciones de conversión de números a letras en ocho diferentes lenguajes de programación del lado del servidor: Ruby, Perl, Node.js, .NET 10, Golang, Java, C++ y Rust.

## Estructura del Proyecto

Para cada lenguaje de programación, se han implementado tres versiones:

### Versión 1: Consumo de Servicio SOAP
Consume el servicio web SOAP público de DataAccess para convertir números a palabras en inglés.

**Servicio:** https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL

### Versión 2: Consumo de Servicio SOAP + Traducción
Consume el servicio web SOAP y traduce el resultado del inglés al español utilizando librerías de traducción.

### Versión 3: Implementación Nativa
Implementa la conversión de números a letras en español utilizando librerías nativas del lenguaje de programación, sin depender de servicios externos.

## Lenguajes Implementados

- **Ruby** - Implementación con Savon y Google Translate
- **Perl** - Implementación con SOAP::Lite y Lingua::Translate
- **Node.js** - Implementación con soap y @google-cloud/translate
- **.NET 10** - Implementación con WCF y Google Cloud Translation
- **Golang** - Implementación con gowsdl y Google Cloud Translation
- **Java** - Implementación con JAX-WS y Google Cloud Translation
- **C++** - Implementación con libcurl y tinyxml2
- **Rust** - Implementación con reqwest

## Instalación y Ejecución

### Ruby
```bash
cd ruby
bundle install
ruby version1.rb 10
ruby version2.rb 10
ruby version3.rb 10
```

### Perl
```bash
cd perl
cpan SOAP::Lite
cpan Lingua::Translate
cpan Lingua::ES::Numeros
perl version1.pl 10
perl version2.pl 10
perl version3.pl 10
```

### Node.js
```bash
cd node
npm install
node version1.js 10
node version2.js 10
node version3.js 10
```

### .NET 10
```bash
cd dotnet
dotnet restore Version1.csproj
dotnet run --project Version1.csproj -- 10
dotnet restore Version2.csproj
dotnet run --project Version2.csproj -- 10
dotnet restore Version3.csproj
dotnet run --project Version3.csproj -- 10
```

### Golang
```bash
cd golang
go mod download
go run version1.go 10
go run version2.go 10
go run version3.go 10
```

### Java
```bash
cd java
mvn install
mvn exec:java -Dexec.mainClass="Version1" -Dexec.args="10"
mvn exec:java -Dexec.mainClass="Version2" -Dexec.args="10"
mvn exec:java -Dexec.mainClass="Version3" -Dexec.args="10"
```

### C++
```bash
cd cpp
# Instalar dependencias (Ubuntu/Debian)
sudo apt-get install libcurl4-openssl-dev libtinyxml2-dev

# Compilar y ejecutar
g++ version1.cpp -lcurl -ltinyxml2 -o version1
./version1 10

g++ version2.cpp -lcurl -ltinyxml2 -o version2
./version2 10

g++ version3.cpp -o version3
./version3 10
```

### Rust
```bash
cd rust
cargo build --bin version1
cargo run --bin version1 -- 10
cargo build --bin version2
cargo run --bin version2 -- 10
cargo build --bin version3
cargo run --bin version3 -- 10
```

## Flujo de Trabajo

Este proyecto utiliza **GitHub Flow** como metodología de control de versiones:

1. Todas las funcionalidades se desarrollan en ramas `feature/`
2. Se realizan pull requests para revisión de código
3. Después de aprobación, se fusionan a la rama `main`
4. La rama `main` siempre está en estado estable

## Documentación

Para más detalles sobre la justificación del uso de Git y GitHub, flujos de trabajo, y configuración, consulte el documento [DOCUMENTACION.md](DOCUMENTACION.md).

## Licencia

Este proyecto es de uso educativo.

## Autor

Desarrollado como proyecto académico para la asignatura de Desarrollo de Software.
