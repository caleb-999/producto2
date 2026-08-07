# Documentación del Proyecto: Conversión de Números a Letras en Múltiples Lenguajes de Programación

---

## Portada

**Universidad**  
**Facultad de Ingeniería**  
**Asignatura:** Desarrollo de Software  
**Profesor:** Javier Nolasco Hernández  
**Título:** Implementación de Clientes SOAP en Múltiples Lenguajes de Programación  
**Fecha:** 19 de Junio de 2026

---

## Índice

1. [Introducción](#introducción)
2. [Justificación del uso de Git y GitHub](#justificación-del-uso-de-git-y-github)
3. [Flujo de trabajo del control de versiones](#flujo-de-trabajo-del-control-de-versiones)
   - 3.1 [Git Flow](#git-flow)
   - 3.2 [GitHub Flow](#github-flow)
4. [Parámetros de configuración de Git y GitHub](#parámetros-de-configuración-de-git-y-github)
5. [Enlace del repositorio en funcionamiento](#enlace-del-repositorio-en-funcionamiento)
6. [Conclusión](#conclusión)

---

## Introducción

El presente proyecto tiene como objetivo principal la implementación de aplicaciones de conversión de números a letras en ocho diferentes lenguajes de programación del lado del servidor: Ruby, Perl, Node.js, .NET 10, Golang, Java, C++ y Rust. Para cada lenguaje se han desarrollado tres versiones de código que progresan desde el consumo de un servicio web SOAP público hasta la implementación nativa de la conversión en español.

El proyecto se basa en el ejemplo de cliente SOAP en PHP presentado en la clase del Lunes 15/06/2026, donde se demostró cómo consumir el servicio web público de DataAccess (https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL) para convertir números a palabras en inglés, y posteriormente traducir el resultado al español utilizando librerías de traducción.

Este proyecto no solo demuestra la versatilidad de diferentes lenguajes de programación para interactuar con servicios web, sino que también establece un flujo de trabajo profesional de control de versiones utilizando Git y GitHub para la gestión del código fuente.

---

## Justificación del uso de Git y GitHub

### Git

Git es un sistema de control de versiones distribuido creado por Linus Torvalds en 2005. Su uso en este proyecto se justifica por las siguientes razones:

1. **Control de Cambios:** Permite rastrear cada modificación realizada en el código fuente, lo cual es fundamental para un proyecto que involucra múltiples versiones por lenguaje y múltiples desarrolladores.

2. **Colaboración:** Facilita el trabajo en equipo permitiendo que múltiples desarrolladores trabajen simultáneamente en diferentes partes del proyecto sin conflictos.

3. **Historial Completo:** Mantiene un historial detallado de todos los commits, permitiendo revertir a versiones anteriores si es necesario y entender la evolución del proyecto.

4. **Ramificación (Branching):** Permite crear ramas para experimentar con nuevas funcionalidades sin afectar el código principal, lo cual es ideal para desarrollar las tres versiones por lenguaje.

5. **Distribución:** Al ser un sistema distribuido, cada desarrollador tiene una copia completa del repositorio, lo que proporciona redundancia y permite trabajar sin conexión a internet.

6. **Eficiencia:** Git está optimizado para ser rápido y eficiente, incluso con proyectos grandes y con muchos archivos.

### GitHub

GitHub es una plataforma de hosting para repositorios Git que proporciona herramientas adicionales para la colaboración. Su justificación en este proyecto incluye:

1. **Accesibilidad:** Permite que el repositorio sea accesible desde cualquier lugar con conexión a internet, facilitando la colaboración remota.

2. **Pull Requests:** Facilita el proceso de revisión de código antes de integrar cambios al repositorio principal.

3. **Issues:** Permite el seguimiento de bugs, tareas y mejoras de manera organizada.

4. **Documentación:** Proporciona herramientas integradas para documentación como README.md, wikis y GitHub Pages.

5. **Integración Continua:** Se puede integrar con herramientas de CI/CD para automatizar pruebas y despliegues.

6. **Comunidad:** Permite que otros desarrolladores contribuyan al proyecto mediante forks y pull requests.

7. **Visibilidad:** Hace el proyecto visible para la comunidad académica y profesional, lo cual es importante para proyectos educativos.

---

## Flujo de trabajo del control de versiones

### Git Flow

Git Flow es un modelo de ramificación extendido que proporciona un marco de trabajo robusto para proyectos con ciclos de lanzamiento programados. Este flujo es especialmente adecuado para este proyecto debido a la naturaleza estructurada de las entregas.

#### Ramas Principales

1. **main (o master):** Esta rama contiene el código de producción. Solo se actualiza cuando se lanza una nueva versión estable del proyecto.

2. **develop:** Esta es la rama principal de desarrollo. Todas las nuevas funcionalidades se integran aquí antes de pasar a producción.

#### Ramas de Soporte

1. **feature/**: Estas ramas se crean a partir de `develop` para desarrollar nuevas funcionalidades. Por ejemplo:
   - `feature/ruby-version1`
   - `feature/perl-version2`
   - `feature/node-version3`

   Cuando la funcionalidad está completa, se fusiona de vuelta a `develop` mediante un pull request.

2. **release/**: Estas ramas se crean a partir de `develop` cuando se prepara una nueva versión para producción. Permiten realizar pruebas finales y correcciones menores sin afectar el desarrollo continuo.

3. **hotfix/**: Estas ramas se crean a partir de `main` para correcciones urgentes en producción.

#### Proceso para este Proyecto

Para este proyecto específico, el flujo de trabajo sería:

1. Crear la rama `develop` desde `main`
2. Para cada lenguaje, crear ramas `feature/nombre-lenguaje-versionX`:
   - `feature/ruby-version1`: Implementar versión 1 de Ruby
   - `feature/ruby-version2`: Implementar versión 2 de Ruby
   - `feature/ruby-version3`: Implementar versión 3 de Ruby
   - Repetir para Perl, Node.js, .NET, Golang, Java, C++, Rust
3. Cada versión se desarrolla en su rama correspondiente
4. Al completar una versión, se hace un pull request a `develop`
5. Después de revisión y aprobación, se fusiona a `develop`
6. Cuando todas las versiones de un lenguaje están completas, se crea una rama `release/nombre-lenguaje`
7. Después de pruebas, se fusiona a `main` y `develop`
8. Se etiqueta (tag) la versión en `main`

### GitHub Flow

GitHub Flow es un modelo de ramificación más simple y ágil, ideal para proyectos con despliegues continuos. Este flujo es más adecuado para proyectos con ciclos de desarrollo más cortos y frecuentes.

#### Ramas Principales

1. **main:** Esta es la rama principal y siempre debe estar en un estado desplegable.

2. **feature/**: Todas las nuevas funcionalidades se desarrollan en ramas de características.

#### Proceso para este Proyecto

Para este proyecto, el flujo de trabajo sería:

1. La rama `main` siempre contiene el código estable
2. Para cada versión de cada lenguaje, crear una rama `feature/nombre-lenguaje-versionX`:
   - `feature/ruby-version1`
   - `feature/ruby-version2`
   - `feature/ruby-version3`
   - Repetir para todos los lenguajes
3. Desarrollar la funcionalidad en la rama de feature
4. Hacer commits frecuentes con mensajes descriptivos
5. Abrir un pull request cuando la funcionalidad esté completa
6. Realizar revisión de código en el pull request
7. Después de aprobación, fusionar a `main`
8. Desplegar desde `main` (si aplica)
9. Eliminar la rama de feature después de la fusión

#### Comparación entre Git Flow y GitHub Flow

| Aspecto | Git Flow | GitHub Flow |
|---------|----------|-------------|
| Complejidad | Más complejo | Más simple |
| Ciclos de lanzamiento | Programados | Continuos |
| Ramas de release | Sí | No |
| Ramas de hotfix | Sí | No |
| Adecuado para | Proyectos grandes | Proyectos ágiles |
| Curva de aprendizaje | Mayor | Menor |

Para este proyecto académico, **GitHub Flow** es más adecuado debido a:
- Simplicidad para estudiantes
- Menos complejidad en la gestión de ramas
- Facilita la entrega progresiva de versiones
- Se alinea mejor con metodologías ágiles

---

## Parámetros de configuración de Git y GitHub

### Configuración de Git

#### Configuración Básica

```bash
# Configurar nombre de usuario
git config --global user.name "Tu Nombre"

# Configurar email
git config --global user.email "tu.email@ejemplo.com"

# Configurar editor predeterminado
git config --global core.editor "code --wait"

# Configurar nombre de rama principal
git config --global init.defaultBranch main

# Ver configuración actual
git config --list
```

#### Configuración Avanzada

```bash
# Configurar manejo de finales de línea (Windows)
git config --global core.autocrlf true

# Configurar manejo de finales de línea (Mac/Linux)
git config --global core.autocrlf input

# Configurar colores en la salida
git config --global color.ui true

# Configurar alias para comandos comunes
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit

# Configurar rebase por defecto al hacer pull
git config --global pull.rebase true
```

#### Configuración de Ignorar Archivos (.gitignore)

```bash
# Crear archivo .gitignore
touch .gitignore

# Contenido típico para este proyecto:
# Ruby
*.gem
*.rbc
.bundle
 Gemfile.lock

# Perl
*.pm~
*.pl~

# Node.js
node_modules/
npm-debug.log
package-lock.json

# .NET
bin/
obj/
*.user
*.suo

# Go
*.exe
*.exe~
*.dll
*.so
*.dylib

# Java
*.class
*.jar
target/

# C++
*.o
*.a

# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
```

### Configuración de GitHub

#### Configuración de Cuenta

1. **Crear cuenta en GitHub:**
   - Visitar https://github.com
   - Registrarse con email y contraseña
   - Verificar email

2. **Configurar perfil:**
   - Nombre completo
   - Bio
   - Foto de perfil
   - Ubicación

3. **Configurar SSH Keys (opcional pero recomendado):**
```bash
# Generar clave SSH
ssh-keygen -t ed25519 -C "tu.email@ejemplo.com"

# Iniciar agente SSH
eval "$(ssh-agent -s)"

# Agregar clave SSH
ssh-add ~/.ssh/id_ed25519

# Copiar clave pública
cat ~/.ssh/id_ed25519.pub
```

Luego agregar la clave en GitHub: Settings → SSH and GPG keys → New SSH key

#### Configuración de Repositorio

1. **Crear repositorio nuevo:**
   - Click en "+" → "New repository"
   - Nombre: `number-conversion-multilang`
   - Descripción: "Proyecto de conversión de números a letras en múltiples lenguajes"
   - Visibilidad: Public
   - Inicializar con README: No (lo haremos localmente)
   - Click en "Create repository"

2. **Conectar repositorio local con GitHub:**
```bash
# Inicializar repositorio Git
git init

# Agregar archivos
git add .

# Primer commit
git commit -m "Initial commit: Estructura del proyecto"

# Agregar remoto
git remote add origin https://github.com/tu-usuario/number-conversion-multilang.git

# O con SSH
git remote add origin git@github.com:tu-usuario/number-conversion-multilang.git

# Push a GitHub
git branch -M main
git push -u origin main
```

#### Configuración de Protección de Ramas

Para proteger la rama principal:

1. Ir al repositorio en GitHub
2. Settings → Branches
3. Add rule
4 - Nombre de rama: `main`
5. Marcar:
   - Require a pull request before merging
   - Require approvals (1)
   - Require status checks to pass before merging
   - Do not allow bypassing the above settings

#### Configuración de Webhooks (Opcional)

Para integración con CI/CD:

1. Settings → Webhooks
2. Add webhook
3. Payload URL: URL del servidor CI/CD
4. Content type: application/json
5. Seleccionar eventos: Push, Pull request

---

## Enlace del repositorio en funcionamiento

El repositorio público del proyecto se encuentra disponible en:

**https://github.com/[TU-USUARIO]/number-conversion-multilang**

### Estructura del Repositorio

```
number-conversion-multilang/
├── ruby/
│   ├── version1.rb
│   ├── version2.rb
│   ├── version3.rb
│   ├── Gemfile
│   └── README.md
├── perl/
│   ├── version1.pl
│   ├── version2.pl
│   ├── version3.pl
│   └── README.md
├── node/
│   ├── version1.js
│   ├── version2.js
│   ├── version3.js
│   ├── package.json
│   └── README.md
├── dotnet/
│   ├── Version1.cs
│   ├── Version2.cs
│   ├── Version3.cs
│   ├── Version1.csproj
│   ├── Version2.csproj
│   ├── Version3.csproj
│   └── README.md
├── golang/
│   ├── version1.go
│   ├── version2.go
│   ├── version3.go
│   ├── go.mod
│   └── README.md
├── java/
│   ├── Version1.java
│   ├── Version2.java
│   ├── Version3.java
│   ├── pom.xml
│   └── README.md
├── cpp/
│   ├── version1.cpp
│   ├── version2.cpp
│   ├── version3.cpp
│   └── README.md
├── rust/
│   ├── version1.rs
│   ├── version2.rs
│   ├── version3.rs
│   ├── Cargo.toml
│   └── README.md
├── DOCUMENTACION.md
└── README.md
```

### Historial de Commits (Ejemplo)

```
* a1b2c3d (HEAD -> main, origin/main) Completar documentación del proyecto
* d4e5f6g Agregar implementación en Rust (versiones 1, 2, 3)
* h7i8j9k Agregar implementación en C++ (versiones 1, 2, 3)
* l1m2n3o Agregar implementación en Java (versiones 1, 2, 3)
* p4q5r6s Agregar implementación en Golang (versiones 1, 2, 3)
* t7u8v9w Agregar implementación en .NET 10 (versiones 1, 2, 3)
* x0y1z2a Agregar implementación en Node.js (versiones 1, 2, 3)
* b3c4d5e Agregar implementación en Perl (versiones 1, 2, 3)
* f6g7h8i Agregar implementación en Ruby (versiones 1, 2, 3)
* j9k0l1m Crear estructura inicial del proyecto
```

### Instrucciones para Clonar y Ejecutar

```bash
# Clonar el repositorio
git clone https://github.com/[TU-USUARIO]/number-conversion-multilang.git
cd number-conversion-multilang

# Ejecutar versión 1 de Ruby
cd ruby
bundle install
ruby version1.rb 10

# Ejecutar versión 2 de Node.js
cd ../node
npm install
node version2.js 10

# Ejecutar versión 3 de Rust
cd ../rust
cargo run --bin version3 -- 10
```

---

## Conclusión

Este proyecto ha demostrado la versatilidad y capacidad de ocho lenguajes de programación diferentes (Ruby, Perl, Node.js, .NET 10, Golang, Java, C++ y Rust) para interactuar con servicios web SOAP y realizar conversiones de números a letras en español. 

La implementación de tres versiones por lenguaje ha permitido explorar diferentes enfoques: desde el consumo directo de servicios web externos, pasando por la integración de librerías de traducción, hasta la implementación nativa de la funcionalidad utilizando las capacidades intrínsecas de cada lenguaje.

El uso de Git y GitHub como herramientas de control de versiones ha proporcionado un marco de trabajo profesional que permite:
- Mantener un historial completo de todos los cambios
- Facilitar la colaboración entre desarrolladores
- Implementar flujos de trabajo estructurados (Git Flow y GitHub Flow)
- Gestionar eficientemente múltiples versiones del código
- Proporcionar visibilidad y accesibilidad al proyecto

La elección de GitHub Flow como metodología principal para este proyecto se justifica por su simplicidad y adecuación para proyectos con ciclos de desarrollo ágiles, lo cual es ideal para entornos educativos y proyectos de desarrollo continuo.

Este proyecto no solo cumple con los objetivos técnicos de implementación de clientes SOAP en múltiples lenguajes, sino que también establece buenas prácticas de desarrollo de software, control de versiones y documentación, preparando a los estudiantes para entornos de desarrollo profesional.

---

**Referencias:**

1. Documentación oficial de Git: https://git-scm.com/doc
2. Documentación oficial de GitHub: https://docs.github.com
3. Servicio SOAP de DataAccess: https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL
4. Git Flow: https://nvie.com/posts/a-successful-git-branching-model/
5. GitHub Flow: https://guides.github.com/introduction/flow/

---

**Fin del Documento**
