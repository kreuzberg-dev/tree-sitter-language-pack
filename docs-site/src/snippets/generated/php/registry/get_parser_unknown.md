```php title="PHP"
<?php

use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;
try {
    TreeSitterLanguagePack::getParser("nonexistent_xyz");
} catch (Throwable $error) {
    echo "Call failed as expected: {$error->getMessage()}\n";
    return;
}
throw new RuntimeException('expected call to fail');

```
