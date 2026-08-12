---
id: fixture_php_folds_query_unknown_language
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
$result = TreeSitterLanguagePack::getFoldsQuery("nonexistent_xyz");

```
