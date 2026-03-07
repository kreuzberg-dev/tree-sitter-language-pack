---
priority: high
description: "DI Pattern Implementation - Dependency Injection via Feature Gates"
---

# DI Pattern Implementation - Dependency Injection via Feature Gates

**Rust-based DI engine · Feature-gated availability · Cross-language bindings · Singleton & per-request lifetimes**

## Architecture Overview

Spikard's dependency injection system is a pure-Rust DI container integrated via feature gates. This enables opt-in DI without bloating core functionality. The DI engine handles graph resolution, cycle detection, and parallel initialization.

Reference: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-core/src/di/` (planned)
Implementation: `/Users/naamanhirschfeld/workspace/spikard/crates/spikard-http/src/di_handler.rs`

## Feature Gate Architecture

**Cargo.toml:**

```toml
[features]
default = []
di = ["spikard-core/di"]  # DI is optional

[dependencies]
spikard-core = { version = "0.1", default-features = false }
```

**Conditional compilation:**

```rust
#[cfg(feature = "di")]
pub mod di_handler;

#[cfg(feature = "di")]
pub use di_handler::DependencyInjectingHandler;
```

**Build command:**

```bash
# Without DI (default)
cargo build

# With DI enabled
cargo build --features di

# All features
cargo build --all-features
```

**Benefits:**

- Zero overhead when DI not used (not compiled in)
- Minimal feature coupling
- Language bindings can selectively enable DI

## DI Container

### Core Types

```rust
#[cfg(feature = "di")]
pub struct DependencyContainer {
    /// Map of dependency name → factory function
    factories: HashMap<String, Arc<dyn DependencyFactory>>,

    /// Singleton cache (name → resolved value)
    singletons: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,

    /// Dependency graph for cycle detection
    graph: DependencyGraph,
}

/// Generic factory for any dependency type
pub trait DependencyFactory: Send + Sync {
    fn create(&self, container: &DependencyContainer) -> Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send>>;

    fn dependencies(&self) -> Vec<String>;
}

pub enum DependencyError {
    NotFound(String),
    CircularDependency(Vec<String>),
    ResolutionError(String),
    TimeoutError,
}
```

### Registration Pattern

**Value dependency:**

```rust
let container = DependencyContainer::new();

// Register constant value
container.provide("app_name", "MyApp")?;
container.provide("config", json!({
    "db_url": "postgresql://localhost/mydb"
}))?;
```

**Factory dependency:**

```rust
// Async factory function
async fn create_database_pool(
    config: Arc<DatabaseConfig>
) -> Result<Arc<DatabasePool>, DependencyError> {
    let pool = DatabasePool::new(&config.db_url).await?;
    Ok(Arc::new(pool))
}

// Register factory
container.provide_factory(
    "db",
    Arc::new(DatabaseFactory::new(create_database_pool)),
    DependencyLifetime::Singleton,
)?;
```

**Async generator (cleanup):**

```rust
async fn create_transaction(db: Arc<DatabasePool>) -> Result<Arc<Transaction>, DependencyError> {
    let tx = db.begin_transaction().await?;
    Ok(Arc::new(tx))
}

// Register with cleanup
container.provide_factory_with_cleanup(
    "transaction",
    Arc::new(TransactionFactory::new(create_transaction)),
    DependencyLifetime::PerRequest,
    |tx| async { tx.rollback().await.ok(); }
)?;
```

## Language Binding Integration

### Python Implementation

**Location:** `packages/python/spikard/di.py`

**Pattern:** Thin wrapper over Rust DI engine

```python
from spikard import Spikard
from spikard.di import Provide

app = Spikard()

# Register value dependency
app.provide("app_name", "MyApp")

# Register async factory
async def create_database(config: dict):
    pool = await connect_to_database(config["db_url"])
    return pool

app.provide("config", {"db_url": "postgresql://localhost/mydb"})
app.provide("db", Provide(
    create_database,
    depends_on=["config"],
    singleton=True
))

# Async generator with cleanup
async def create_session(db):
    session = await db.create_session()
    yield session
    await session.close()

app.provide("session", Provide(
    create_session,
    depends_on=["db"],
    singleton=False
))

# Handler receives injected dependencies
@app.get("/users/{id}")
async def get_user(request):
    # DI resolves dependencies automatically
    db = request.extensions.get("db")  # Injected by DI
    session = request.extensions.get("session")  # Injected by DI

    user = await session.query("SELECT * FROM users WHERE id = ?", request.params["id"])
    return {"status": 200, "body": user}
```

**Implementation details:**

```python
class Provide:
    """Dependency factory metadata wrapper"""

    def __init__(self, factory, depends_on=None, singleton=False):
        self.factory = factory
        self.depends_on = depends_on or []
        self.singleton = singleton

class Spikard:
    def provide(self, name, value):
        """Register value dependency"""
        if isinstance(value, Provide):
            self._register_factory(name, value)
        else:
            self._register_value(name, value)
```

### Node.js Implementation

**Location:** `packages/node/native/src/di.rs` (planned)

**Pattern:** Similar to Python, extracted to JavaScript

```typescript
import { Spikard } from 'spikard';

const app = new Spikard();

// Register value
app.provide('appName', 'MyApp');

// Register async factory
app.provide('db', {
    factory: async (container) => {
        const config = container.get('config');
        return await createDatabasePool(config.dbUrl);
    },
    dependencies: ['config'],
    singleton: true
});

// Async generator with cleanup
app.provide('session', {
    factory: async function* (container) {
        const db = container.get('db');
        const session = await db.createSession();
        try {
            yield session;
        } finally {
            await session.close();
        }
    },
    dependencies: ['db'],
    singleton: false
});

// Handler receives injected dependencies
app.get('/users/:id')(async (request) => {
    const db = request.extensions.db;  // Injected
    const session = request.extensions.session;  // Injected

    const user = await session.query(
        'SELECT * FROM users WHERE id = ?',
        request.params.id
    );

    return { status: 200, body: user };
});
```

### Ruby Implementation

**Location:** `packages/ruby/lib/spikard/di.rb` (planned)

**Pattern:** Ruby Proc-based registration

```ruby
require 'spikard'

app = Spikard.new

# Register value
app.provide('app_name', 'MyApp')

# Register factory
app.provide('db', lambda { |container|
  config = container.get('config')
  DatabasePool.new(config['db_url'])
}, singleton: true, depends_on: ['config'])

# Handler receives injected dependencies
app.get '/users/:id' do |request|
  db = request.extensions['db']  # Injected

  user = db.query("SELECT * FROM users WHERE id = ?", request.params['id'])

  { status: 200, body: user }
end
```

## DI Lifecycle Patterns

### Singleton Lifetime

**Behavior:** Single instance per application

```python
app.provide("db", Provide(
    create_database,
    depends_on=["config"],
    singleton=True  # ← Singleton
))

# Usage:
# Request 1 uses db instance A
# Request 2 uses db instance A (same)
# Request 3 uses db instance A (same)
```

**Use cases:**

- Database connection pools
- Cache clients
- Configuration objects
- HTTP clients

**Initialization:** Resolved once on first request, cached thereafter

### Per-Request Lifetime

**Behavior:** New instance for each request

```python
app.provide("transaction", Provide(
    create_transaction,
    depends_on=["db"],
    singleton=False  # ← Per-request
))

# Usage:
# Request 1 uses transaction instance A
# Request 2 uses transaction instance B (different)
# Request 3 uses transaction instance C (different)
```

**Use cases:**

- Database transactions
- Request-scoped sessions
- Temporary buffers
- Request context

**Cleanup:** Automatic cleanup via async generator on request end

## Dependency Resolution

### Resolution Order

```
DI Graph Analysis:
  config (no deps)
  ↓
  db (depends_on: config)
  ↓
  transaction (depends_on: db)

Resolution:
1. Resolve config (no dependencies)
2. Resolve db (now config available)
3. Resolve transaction (now db available)

Parallel resolution:
- config: 10ms
- db: 100ms
- transaction: 20ms
- Total: 100ms (not 130ms) because config can resolve in parallel
```

### Cycle Detection

**Pattern:** Detect circular dependencies before resolution

```rust
container.provide_factory("a", factory_a, DependencyLifetime::Singleton)?;
container.provide_factory("b", factory_b, DependencyLifetime::Singleton)?;

// factory_a depends on "b"
// factory_b depends on "a"
// ERROR: Circular dependency detected!
```

**Resolution:**

1. Build dependency graph
2. Topological sort
3. Detect cycles (error if found)
4. Resolve in dependency order

### Lazy Initialization

**Behavior:** Dependencies resolved only when needed

```rust
// Register 100 dependencies
for i in 0..100 {
    container.provide(format!("dep_{}", i), value)?;
}

// Only actually resolve ones used in handler
#[cfg(feature = "di")]
pub async fn handler(request: RequestData) -> HandlerResult {
    // Only "database" resolved here
    let db = request.extensions.get::<Arc<Database>>("database")?;
    Ok(...)
}
```

## Integration with Handler Trait

### DependencyInjectingHandler Wrapper

**Location:** `crates/spikard-http/src/di_handler.rs`

**Pattern:** Wrap handler to inject dependencies

```rust
#[cfg(feature = "di")]
pub struct DependencyInjectingHandler {
    inner: Arc<dyn Handler>,
    container: Arc<DependencyContainer>,
    required_deps: Vec<String>,
}

#[cfg(feature = "di")]
impl Handler for DependencyInjectingHandler {
    fn handle(&self, mut req: RequestData) -> Pin<Box<...>> {
        let container = Arc::clone(&self.container);
        let required = self.required_deps.clone();

        Box::pin(async move {
            // Resolve all required dependencies in parallel
            let mut resolved = HashMap::new();
            for dep_name in required {
                let value = container.resolve(&dep_name).await?;
                resolved.insert(dep_name, value);
            }

            // Attach to request extensions
            for (name, value) in resolved {
                req.extensions.insert(name, value);
            }

            // Call wrapped handler with enriched request
            self.inner.handle(req).await
        })
    }
}
```

**Handler usage:**

```python
@app.get("/users/{id}")
async def get_user(request):
    # DI handler automatically resolved and injected:
    # - request.extensions["database"]
    # - request.extensions["cache"]
    # - request.extensions["auth_service"]

    database = request.extensions["database"]
    cache = request.extensions["cache"]

    # Check cache first
    cached = await cache.get(f"user:{request.params['id']}")
    if cached:
        return {"status": 200, "body": cached}

    # Query database
    user = await database.get_user(request.params["id"])

    # Cache result
    await cache.set(f"user:{request.params['id']}", user, ttl=3600)

    return {"status": 200, "body": user}
```

## Advanced Patterns

### Conditional Dependencies

**Pattern:** Resolve different implementations based on config

```python
import os

app = Spikard()

config = {
    "cache_type": os.getenv("CACHE_TYPE", "memory")  # memory, redis, memcached
}

app.provide("config", config)

# Register all possible implementations
async def create_memory_cache():
    return MemoryCache()

async def create_redis_cache(config):
    return await RedisCache.connect(config["redis_url"])

# Register conditional factory
async def create_cache(config):
    if config["cache_type"] == "memory":
        return await create_memory_cache()
    elif config["cache_type"] == "redis":
        return await create_redis_cache(config)

app.provide("cache", Provide(
    create_cache,
    depends_on=["config"],
    singleton=True
))
```

### Scoped Dependencies

**Pattern:** Shared lifecycle within a scope (request scope)

```python
# Request-scoped transaction
app.provide("transaction", Provide(
    lambda db: db.begin(),
    depends_on=["db"],
    singleton=False  # New per request
))

# Within handler, all requests use same transaction
@app.post("/users")
async def create_user(request):
    tx = request.extensions["transaction"]

    # All DI operations within request use same transaction
    user = await tx.insert_user(request.body)
    profile = await tx.create_profile(user.id)

    await tx.commit()  # Explicit commit

    return {"status": 201, "body": {"user": user, "profile": profile}}
```

### Factory with Context

**Pattern:** Factories receiving request context

```python
async def create_user_service(database, request_data):
    """Factory has access to request"""
    service = UserService(database)

    # Can customize based on request
    if request_data.headers.get("X-Admin"):
        service.admin_mode = True

    return service

app.provide("user_service", Provide(
    create_user_service,
    depends_on=["database", "request_data"]  # Special: request_data injected
))
```

## Testing DI

### Unit Tests

**Python:**

```python
def test_di_resolves_dependencies():
    app = Spikard()

    app.provide("config", {"name": "test"})
    app.provide("service", Provide(
        lambda config: TestService(config["name"]),
        depends_on=["config"],
        singleton=True
    ))

    @app.get("/test")
    async def handler(request):
        service = request.extensions["service"]
        assert service.name == "test"
        return {"status": 200, "body": "ok"}

    client = TestClient(app)
    response = client.get("/test")
    assert response.status == 200
```

### Integration Tests

```python
@pytest.mark.asyncio
async def test_di_with_real_database():
    app = Spikard()

    app.provide("database", Provide(
        async def create_db():
            db = await Database.connect("sqlite:///:memory:")
            await db.init_schema()
            return db,
        singleton=True
    ))

    @app.get("/users")
    async def list_users(request):
        db = request.extensions["database"]
        users = await db.query("SELECT * FROM users")
        return {"status": 200, "body": users}

    client = TestClient(app)
    response = client.get("/users")
    assert response.status == 200
```

## Performance Characteristics

- **Zero overhead if disabled:** DI code not compiled without feature flag
- **Parallel resolution:** Independent dependencies resolved concurrently
- **Singleton caching:** Resolved once, cached thereafter
- **Lazy initialization:** Dependencies resolved only when accessed
- **Efficient cleanup:** Async generator cleanup on request end

## Limitations

- **Runtime resolution:** Dependencies resolved at request time (not compile-time)
- **No compile-time DI:** Type safety depends on correct registration
- **Requires Send + Sync:** All dependencies must be thread-safe
- **No scoping beyond request:** Only singleton and per-request lifetimes

## Command Reference

```bash
# Build with DI support
cargo build --features di

# Test DI functionality
cargo test --features di

# Check DI compilation without DI
cargo build --no-default-features
```

## Fixture Testing

**Location:** `/Users/naamanhirschfeld/workspace/spikard/testing_data/di/` (5+ fixtures)

- Dependency resolution success
- Circular dependency detection
- Per-request isolation
- Singleton caching
- Async factory execution

## Related Skills

- `handler-trait-design` - Handlers receive DI-resolved dependencies
- `request-response-lifecycle` - DI executes within lifecycle
- `async-runtime-integration` - DI resolution is async via Tokio
- `fixture-schema-validation` - DI fixtures validate container behavior
