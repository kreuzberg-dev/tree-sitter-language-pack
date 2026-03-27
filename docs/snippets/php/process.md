```php title="PHP"
<?php
$resultJson = \ts_pack_process(
    '<?php namespace App; class Controller { public function index() {} }',
    '{"language":"php","structure":true,"imports":true}'
);
$result = json_decode($resultJson, true);
print_r($result);
```
