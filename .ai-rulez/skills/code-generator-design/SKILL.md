---
priority: high
description: "Code Generator Design - Spec-to-Handler Codegen"
---

# Code Generator Design - Spec-to-Handler Codegen

**OpenAPI/AsyncAPI codegen · Handler stubs · Type-safe bindings · Test fixtures**

## Architecture Overview

Spikard's code generation pipeline transforms API specifications into working handler implementations across all language bindings. This enables contract-first API development with zero boilerplate.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-codegen/`

## Specification Support

### OpenAPI 3.0.x

**Location:** `crates/spikard-http/src/openapi/`

- Full OpenAPI 3.0.0-3.1.0 support
- Automatic schema detection from handlers
- Info object (title, version, description)
- Server objects (URLs, variables)
- Security schemes (JWT, API Key, OAuth2, Basic Auth)
- Contact and License information

**Configuration:**

```rust
pub struct OpenApiConfig {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub base_path: Option<String>,
    pub servers: Vec<ServerInfo>,
    pub security_schemes: Vec<(String, SecuritySchemeInfo)>,
    pub contact: Option<ContactInfo>,
    pub license: Option<LicenseInfo>,
}
```

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/openapi_schemas/` (10+ OpenAPI documents)

### AsyncAPI 2.0.0

**Location:** `crates/spikard-http/src/` (integration with lifecycle hooks)

- Event schema definitions
- Message payload specifications
- Subscription/publication patterns
- Server configuration for WebSocket

**Fixtures:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/asyncapi_schemas/` (5+ AsyncAPI specs)

### JSON Schema

**Location:** All specifications use JSON Schema for validation

- Request body schemas
- Response body schemas
- Parameter schemas (path, query, header)
- Reusable component definitions
- Validation via `SchemaRegistry` and `SchemaValidator`

**Fixtures:** `testing_data/json_bodies/`, `testing_data/validation_errors/` (35+ schema validation tests)

## Codegen Pipeline

### Stage 1: Specification Parsing

**Input:** OpenAPI/AsyncAPI YAML/JSON

**Process:**

1. Parse YAML/JSON to AST
2. Resolve `$ref` references (components, external refs)
3. Validate spec against OpenAPI/AsyncAPI schema
4. Extract path items, schemas, security definitions

**Tools:**

- YAML parser: `serde_yaml`
- JSON parser: `serde_json`
- Schema validation: custom validator

### Stage 2: Type Extraction

**Input:** Parsed specification

**Process:**

1. Walk paths and extract HTTP methods
2. Extract parameter definitions (path, query, header)
3. Extract request body schemas
4. Extract response schemas
5. Map to Rust types

**Type Mapping:**

```
OpenAPI Type         Rust Type          JavaScript Type    Python Type
string               String             string             str
integer              i64                number             int
number               f64                number             float
boolean              bool               boolean            bool
object               serde_json::Value  object             dict
array                Vec<T>             Array              list
```

### Stage 3: Handler Stub Generation

**Output:** Handler templates in target language

#### Node.js Handler Stubs

```typescript
// Generated from OpenAPI spec
import { Spikard, get, post, RequestData } from 'spikard';

const app = new Spikard();

/**
 * Get user by ID
 * @param request Request object
 * @returns User object
 */
get('/users/{id}')((request: RequestData) => {
  const userId = request.params.id;  // auto-typed as string

  return {
    status: 200,
    headers: { 'Content-Type': 'application/json' },
    body: {
      id: userId,
      name: 'Generated Handler',
      email: 'user@example.com'
    }
  };
});
```

#### Python Handler Stubs

```python
# Generated from OpenAPI spec
from spikard import Spikard
from typing import Optional

app = Spikard()

@app.get('/users/{id}')
async def get_user(request) -> dict:
    """Get user by ID"""
    user_id = request.params['id']  # auto-typed

    return {
        'status': 200,
        'headers': {'Content-Type': 'application/json'},
        'body': {
            'id': user_id,
            'name': 'Generated Handler',
            'email': 'user@example.com'
        }
    }
```

#### Ruby Handler Stubs

```ruby
# Generated from OpenAPI spec
require 'spikard'

app = Spikard.new

app.get('/users/{id}') do |request|
  user_id = request.params['id']

  {
    status: 200,
    headers: { 'Content-Type' => 'application/json' },
    body: {
      id: user_id,
      name: 'Generated Handler',
      email: 'user@example.com'
    }
  }
end
```

### Stage 4: Validation Schema Extraction

**Output:** JSON Schema validators

**Process:**

1. Extract request schemas → validation rules
2. Extract response schemas → documentation
3. Generate parameter validators for path/query params
4. Create composite validators

**Implementation:** `crates/spikard-core/src/schema.rs`

```rust
pub struct SchemaValidator {
    pub request_schema: Option<JsonSchema>,
    pub response_schemas: HashMap<u16, JsonSchema>,
    pub parameter_validators: Vec<ParameterValidator>,
}
```

### Stage 5: Test Fixture Generation

**Output:** Parametrized test cases

**Process:**

1. Extract examples from OpenAPI spec
2. Generate test fixtures matching schema
3. Create both valid and invalid test cases
4. Generate expected responses

**Directory Structure:**

```
testing_data/
├── 00-FIXTURE-SCHEMA.json         # JSON Schema for all fixtures
├── headers/                        # 10+ header test cases
├── json_bodies/                    # 15+ body validation fixtures
├── path_params/                    # Parameter type coercion
├── query_params/                   # Query string parsing
├── validation_errors/              # 422 response examples
├── status_codes/                   # All HTTP status codes
├── [method specific]/              # GET, POST, PUT, etc.
└── [content type specific]/        # JSON, form, multipart
```

## Fixture Schema Validation

**Location:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/00-FIXTURE-SCHEMA.json`

**Meta-schema enforcing all fixtures:**

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Spikard Test Fixture Schema",
  "type": "object",
  "properties": {
    "category": { "type": "string" },
    "method": { "enum": ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"] },
    "path": { "type": "string" },
    "request": {
      "type": "object",
      "properties": {
        "headers": { "type": "object" },
        "body": { "oneOf": [{ "type": "object" }, { "type": "null" }] },
        "query": { "type": "object" },
        "pathParams": { "type": "object" }
      }
    },
    "response": {
      "type": "object",
      "properties": {
        "status": { "type": "integer" },
        "headers": { "type": "object" },
        "body": { "oneOf": [{ "type": "object" }, { "type": "null" }] }
      },
      "required": ["status"]
    }
  },
  "required": ["method", "path", "request", "response"]
}
```

## Integration with Language Bindings

### Node.js

**Auto-generation workflow:**

1. Load OpenAPI spec from `spec.yaml`
2. Run `spikard codegen --spec spec.yaml --lang typescript --output src/handlers.generated.ts`
3. Generated file includes:
   - Type definitions (RequestData, route-specific params)
   - Handler stubs for each path
   - Validation functions
   - Test fixture imports
4. Import and extend generated handlers:

   ```typescript
   import { getUser as generatedGetUser } from './handlers.generated';

   export const getUser = generatedGetUser;
   ```

### Python

**Auto-generation workflow:**

1. Load OpenAPI spec from `spec.yaml`
2. Run `spikard codegen --spec spec.yaml --lang python --output handlers_generated.py`
3. Generated file includes:
   - Type hints (TypedDict for RequestData)
   - Handler stubs
   - Validation decorators
   - Test parametrization data
4. Extend with actual logic:

   ```python
   from handlers_generated import get_user as generated_get_user

   @app.get('/users/{id}')
   async def get_user(request):
       # Implement actual logic
       return await generated_get_user(request)
   ```

### Ruby

**Auto-generation workflow:**

1. Load OpenAPI spec from `spec.yaml`
2. Run `spikard codegen --spec spec.yaml --lang ruby --output handlers_generated.rb`
3. Use generated routes and documentation

## Test Fixture Auto-Generation

### Command Line Tool

```bash
# Generate test fixtures from OpenAPI spec
spikard codegen-fixtures --spec api.yaml --output testing_data/

# Generates:
# - testing_data/headers/*.json (all header combinations)
# - testing_data/json_bodies/*.json (all schema examples)
# - testing_data/query_params/*.json (parameter types)
# - testing_data/path_params/*.json (path coercion)
# - testing_data/validation_errors/*.json (422 responses)
```

### Fixture Generation Strategy

**For each path:**

1. **Happy Path Fixtures:**
   - Valid request with all required fields
   - Expected 2xx response
   - Include example from spec

2. **Validation Error Fixtures:**
   - Missing required fields
   - Wrong type for field
   - Expected 422 response with error details

3. **Parameter Type Fixtures:**
   - String → string
   - Number → integer/float
   - Boolean → boolean
   - Array → proper serialization

4. **Content-Type Fixtures:**
   - JSON body tests
   - Form-urlencoded tests
   - Multipart with files
   - Empty bodies

**Example generated fixture:**

```json
{
  "category": "json_bodies",
  "name": "user_creation_valid",
  "description": "Valid user creation request",
  "method": "POST",
  "path": "/users",
  "request": {
    "headers": {
      "Content-Type": "application/json"
    },
    "body": {
      "name": "Alice",
      "email": "alice@example.com",
      "age": 30
    }
  },
  "response": {
    "status": 201,
    "headers": {
      "Content-Type": "application/json"
    },
    "body": {
      "id": 1,
      "name": "Alice",
      "email": "alice@example.com",
      "age": 30,
      "created_at": "2024-01-15T10:30:00Z"
    }
  }
}
```

## Language-Agnostic Features

### Error Response Generation

All codegen targets produce consistent error schemas:

```json
{
  "type": "https://example.com/problems/validation-error",
  "title": "Validation Failed",
  "status": 422,
  "detail": "One or more validation errors",
  "errors": [
    {
      "field": "email",
      "message": "Invalid email format"
    }
  ]
}
```

### Schema References

Codegen handles `$ref` resolution:

```yaml
components:
  schemas:
    User:
      type: object
      properties:
        id: { type: integer }
        name: { type: string }

paths:
  /users:
    post:
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/User'
```

Codegen:

1. Resolves `$ref` to `User` schema
2. Generates validation for all User properties
3. Generates test fixtures with valid User objects

## Current Limitations

- **GraphQL:** Limited integration (custom endpoint codegen)
- **WebSocket:** Not yet codegen-supported
- **Streaming:** Limited fixture support (SSE works, WebSocket pending)
- **External refs:** HTTP refs to external specs (partial support)

## Command Reference

```bash
# Generate handler stubs from OpenAPI spec
spikard codegen \
  --spec api.yaml \
  --lang typescript|python|ruby|wasm \
  --output src/handlers.generated.ts

# Generate test fixtures
spikard codegen-fixtures \
  --spec api.yaml \
  --output testing_data/

# Validate spec
spikard validate-spec api.yaml

# Generate OpenAPI from running app
spikard openapi:generate \
  --app src/main.ts \
  --output api.yaml
```

## Related Skills

- `handler-trait-design` - Handlers match generated signatures
- `fixture-schema-validation` - Fixtures follow generated schema
- `tower-middleware-patterns` - Codegen respects middleware config
- `request-response-lifecycle` - Lifecycle hooks in generated handlers
