---
id: fixture_php_error_handling_get_language_empty_string
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
try {
    TreeSitterLanguagePack::getLanguage("");
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
