---
priority: high
---

# Code Generation Domain

**OpenAPI, GraphQL, AsyncAPI, OpenRPC handler generation**

## Overview

Spikard provides comprehensive code generation capabilities for multiple specification formats (OpenAPI 3.0+, GraphQL SDL, AsyncAPI 3.0, OpenRPC), generating language-specific handlers that implement the unified Handler trait.

## Core Components

### OpenAPI Codegen

Located in `/crates/spikard-http/src/openapi/` and `/crates/spikard-codegen/src/openapi/`:

- **Spec Generation** (`spec_generation.rs`): Converts Route definitions to OpenAPI 3.0 specs
- **Parameter Extraction** (`parameter_extraction.rs`): Extracts path/query/header/cookie parameters from routes
- **Schema Conversion** (`schema_conversion.rs`): Converts JSON schemas to OpenAPI schemas
- **OpenAPI Config**: Metadata (title, version, description, contact, license, servers)
- **Integration**: Swagger UI and ReDoc UI embedded via utoipa

### GraphQL Support

Located in `/crates/spikard-graphql/src/`:

- **Schema** (`schema.rs`): GraphQL schema definition and type system
- **Executor** (`executor.rs`): Query and mutation execution
- **Handler** (`handler.rs`): GraphQL HTTP handler (POST /graphql)
- **Routes** (`routes.rs`): Introspection routes and schema endpoint

### AsyncAPI Support

Located in `/testing_data/asyncapi_schemas/`:

- AsyncAPI 3.0 specification parsing
- Event-driven architecture definitions
- Channel and operation extraction
- Message payload schema generation

### OpenRPC Support

Located in `/tools/test-generator/`:

- JSON-RPC 2.0 method registry generation
- Method schema extraction
- Parameter validation schema generation

### Test Fixture Integration

Located in `/testing_data/`:

- `/openapi_schemas/` - OpenAPI spec examples
- `/graphql/` - GraphQL schema and query fixtures
- `/asyncapi_schemas/` - AsyncAPI spec examples
- `/jsonrpc/validation/` - JSON-RPC validation fixtures

## Handler Generation Pipeline

1. **Specification Parsing**: Parse OpenAPI/GraphQL/AsyncAPI/OpenRPC specs
2. **Type Extraction**: Extract operation/field/method signatures
3. **Schema Validation**: Generate validators from schemas
4. **Language Binding Code Generation**: Generate Python/Node/Ruby/PHP/WASM code
5. **Handler Implementation**: Generated code implements Handler trait
6. **Testing**: Fixture-driven tests validate generated handlers

## Key Files

- `/crates/spikard-codegen/src/lib.rs` - Codegen entry point
- `/crates/spikard-codegen/src/openapi/mod.rs` - OpenAPI codegen utilities
- `/crates/spikard-codegen/src/openapi/spec.rs` - Spec parsing
- `/crates/spikard-codegen/src/openapi/from_fixtures.rs` - Fixture-based spec generation
- `/crates/spikard-http/src/openapi/mod.rs` - OpenAPI spec generation from routes
- `/crates/spikard-http/src/openapi/spec_generation.rs` - Route to OpenAPI conversion
- `/crates/spikard-graphql/src/lib.rs` - GraphQL integration
- `/crates/spikard-graphql/src/routes.rs` - GraphQL route definitions
- `/tools/test-generator/` - Fixture and test generation utilities

## Specification Support Matrix

| Format   | Parser      | Codegen | Tests               |
|----------|-------------|---------|---------------------|
| OpenAPI  | openapiv3   | Yes     | /testing_data/openapi_schemas/ |
| GraphQL  | graphql-parser | Yes  | /testing_data/graphql/        |
| AsyncAPI | asyncapiv3  | Yes     | /testing_data/asyncapi_schemas/ |
| OpenRPC  | Custom      | Yes     | /testing_data/jsonrpc/        |

## Integration with HTTP Framework

- OpenAPI specs are generated from Route definitions
- GraphQL handler implements Handler trait
- Routes are registered with Router
- Middleware (auth, validation) applies to generated handlers
- Lifecycle hooks execute before/after generated handler logic

## Language Binding Workflow

1. **Binding receives** Route definition with schema
2. **Binding parses** specification (GraphQL schema, OpenAPI operation)
3. **Binding generates** type-safe parameter extraction
4. **Binding wraps** in Handler implementation
5. **Binding registers** with HTTP server

Each binding (Python, Node, Ruby, PHP, WASM) has language-specific code generation:

- Type definitions from schemas
- Parameter validators
- Response serializers
- Error handlers

## Testing Strategy

- Fixture-based tests in `/testing_data/`
- Schema validation tests ensure consistency
- Cross-language tests verify same behavior in all bindings
- Round-trip tests: spec → code → spec
