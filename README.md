# Implementación de API REST para gestión de tareas

Construye una API REST en Rust utilizando Actix Web para gestionar tareas. La API debe permitir crear, listar y actualizar tareas. Utiliza serde para la serialización y un repositorio en memoria. Los errores de dominio deben mapearse a respuestas HTTP con el código correcto.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | API REST de tareas con Rust y Actix |
| **Nivel** | senior-l2 |
| **Tipo** | practical |
| **Tiempo estimado** | 12 horas |

## Stack

- **Lenguaje:** Rust
- **Framework:** Actix Web 4.9
- **Version:** Rust 1.75+

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Tener el proyecto base del reto funcionando en tu máquina: clonar el repositorio, instalar las herramientas y verificar que compila y arranca antes de empezar a practicar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Instala las herramientas para ejecutar el proyecto: Rust (rustup) y Cargo.
- Clona el repositorio del reto (el enlace viene en la entrega del reto y en la sección "Repositorio del Proyecto" del PDF) y ábrelo en tu IDE.
- Instala las dependencias y compila ejecutando `cargo check` en la raíz del repositorio.
- Confirma que el comando termina sin errores: el proyecto arranca y sus pruebas corren.
- Ubica los archivos de práctica: son los que contienen `TODO(reto)` (búscalos en el repositorio). Lee el README.md y el AGENTS.md antes de empezar.

**Entregable:** El proyecto compila/arranca sin errores con `cargo check`.

<details>
<summary>Pistas de conocimiento</summary>

- Los archivos con `TODO(reto)` son el ejercicio: el resto del repositorio ya está resuelto y no hace falta tocarlo.
- Si el comando de verificación falla, revisa la versión de las herramientas instaladas antes de modificar código.
- AGENTS.md describe el reto, el comando de verificación y las reglas del repositorio.

</details>

### Fase 1: Configuración inicial y creación de tareas

**Objetivo:** Configurar el entorno y crear el endpoint para añadir nuevas tareas.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Configura el proyecto Rust con Actix Web.
- Implementa el endpoint POST /tasks para crear una nueva tarea.
- Asegúrate de que la tarea creada tenga un identificador único y que se almacene en un repositorio en memoria.

**Entregable:** Endpoint POST /tasks funcional que crea y almacena tareas en memoria.

<details>
<summary>Pistas de conocimiento</summary>

- Recuerda usar serde para la serialización de las tareas.
- Considera cómo manejarás los errores de validación en este endpoint.

</details>

### Fase 2: Listado y actualización de tareas

**Objetivo:** Implementar endpoints para listar y actualizar tareas existentes.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Implementa el endpoint GET /tasks para listar todas las tareas almacenadas.
- Implementa el endpoint PUT /tasks/{id} para actualizar una tarea existente.
- Asegúrate de que los endpoints manejen correctamente los errores de dominio, mapeándolos a respuestas HTTP con el código correcto.

**Entregable:** Endpoints GET /tasks y PUT /tasks/{id} funcionales que listan y actualizan tareas, respectivamente.

<details>
<summary>Pistas de conocimiento</summary>

- Recuerda mapear los errores de dominio a respuestas HTTP adecuadas.
- Considera cómo manejarías una tarea no encontrada al intentar actualizarla.

</details>

### Fase 3: Optimización y consideraciones finales

**Objetivo:** Optimizar la API y considerar decisiones de diseño.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Evalúa la API implementada y considera posibles optimizaciones.
- Documenta las decisiones de diseño tomadas y justifica las elecciones realizadas.
- Considera cómo podrías mejorar la escalabilidad y la seguridad de la API.

**Entregable:** Documentación que detalla las decisiones de diseño y posibles optimizaciones para la API.

<details>
<summary>Pistas de conocimiento</summary>

- Considera el uso de patrones de diseño para mejorar la estructura de la API.
- Piensa en cómo podrías implementar autenticación y autorización en la API.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una API REST y cómo se implementa en Rust con Actix Web?
- **paraQueSirve**: ¿Para qué sirve el endpoint POST /tasks en el contexto de la gestión de tareas?
- **comoSeUsa**: ¿Cómo se usa serde para la serialización de datos en Rust?
- **erroresComunes**: ¿Cuáles son los errores comunes al implementar una API REST y cómo se manejan en este reto?
- **queDecisionesImplica**: ¿Qué decisiones de diseño implica la implementación de una API REST en Rust con Actix Web?

## Criterios de Evaluacion

- Implementación correcta del endpoint POST /tasks.
- Implementación correcta del endpoint GET /tasks.
- Implementación correcta del endpoint PUT /tasks/{id}.
- Uso correcto de serde para la serialización.
- Manejo correcto de errores de dominio mapeados a respuestas HTTP.
- Documentación de decisiones de diseño y posibles optimizaciones.

## Como trabajar con un asistente de IA

Hay dos caminos, elegi uno:

- **AGENTS.md** (recomendado) — instrucciones nativas del repo. Abri esta carpeta con tu agente local (Claude Code, Cursor, Codex, Copilot, Gemini) y las carga solo. Sabe que archivos faltan y con que comando se verifica, y completa el scaffold escribiendo en disco.
- **PROMPT_MEJORA.md** — para copiar y pegar en un chat (claude.ai, ChatGPT) si no tenes un agente en el IDE. Declara lo mismo que AGENTS.md; los archivos que te devuelva el chat los guardas en esta carpeta respetando las rutas.

Ninguno de los dos resuelve las fases del reto: eso es tu trabajo.

## Verificacion

El proyecto esta listo para trabajar cuando este comando corre sin errores:

```bash
cargo check
```

---

*Reto generado automaticamente por Challenge Generator - Pragma*
