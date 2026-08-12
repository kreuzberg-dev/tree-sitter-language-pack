---
id: fixture_php_detect_content_python_shebang
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
$result = TreeSitterLanguagePack::detectLanguageFromContent("#!/usr/bin/env python3\npass");

```
