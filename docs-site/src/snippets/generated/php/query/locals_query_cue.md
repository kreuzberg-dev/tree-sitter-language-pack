---
id: fixture_php_locals_query_cue
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
$result = TreeSitterLanguagePack::getLocalsQuery("cue");

```
