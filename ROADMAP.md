# ROADMAP

`rust-compiler-design` recorre el pipeline de un compilador para enseñar cómo
una cadena de caracteres llega a ser un resultado ejecutable. No tiene fechas
límite: registra dirección y criterios de calidad, no una carrera por terminar
(RFC-0001 §1).

## Estado Actual

La fundación y el plan operativo están en preparación. Antes de código se
crearán el GitHub Project, milestones e issues asignados a `jeresoftx`.

## Dirección Técnica

1. Frontend: tokens, sintaxis y AST.
2. Semántica: nombres, tipos y diagnósticos.
3. Ejecución: IR, optimización, bytecode y máquina virtual.
4. Integración: programas de ejemplo, ruta de lectura y auditoría editorial.

## Fuera De Alcance Por Ahora

- Usar `unsafe`, nightly o dependencias no triviales sin autorización humana.
- Convertir el lenguaje educativo en un compilador de producción o añadir
  sintaxis sin una pregunta pedagógica concreta.
- Reexplicar capítulos canónicos de estructuras, testing o APIs.
- Marcar contenido como revisado o publicado sin revisión humana.
