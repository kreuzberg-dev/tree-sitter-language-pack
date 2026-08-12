---
id: fixture_php_download_configure_custom_dir
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
use Tree\Sitter\Language\Pack\Language;
use Tree\Sitter\Language\Pack\PackConfig;
use Tree\Sitter\Language\Pack\Tree;
$config = \Tree\Sitter\Language\Pack\PackConfig::from_json(json_encode(["cacheDir" => "/tmp/tslp_test_cache"]));
TreeSitterLanguagePack::configure($config);

```
