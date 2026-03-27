```php title="PHP"
<?php
if (\ts_pack_has_language("php")) {
    $sexp = \ts_pack_parse_string("php", '<?php function hello() { echo "world"; } ?>');
    echo "Tree: $sexp\n";
}
```
