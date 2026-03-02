# Academic Management – Contexto del Proyecto

## Estado Actual

- Base de datos PostgreSQL creada
- Tablas: role, user, career, course, enrollment
- Proyecto Rust creado
- Arquitectura limpia base definida
- Conexión a PostgreSQL funcionando
- Endpoint /health operativo

## Estructura Actual

domain/
application/
infrastructure/
adapters/
errors/

## Próximo Objetivo

Módulo 02 – Routing

- Crear rutas modulares
- Implementar /users
- Separar handlers
- Aplicar nesting con Router::nest()
- Preparar integración futura con AppState

## Preguntas a Resolver

- Cómo organizar routers por feature
- Dónde inyectar AppState
- Cómo preparar rutas para auth futura