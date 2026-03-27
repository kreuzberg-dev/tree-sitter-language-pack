```php title="PHP"
<?php
\ts_pack_init('{"languages":["php","javascript"]}');
\ts_pack_download(["python", "rust"]);

$cached = \ts_pack_downloaded_languages();
print_r($cached);
```
