```bash title="CLI"
# Download specific languages
ts-pack download python javascript rust go

# Download all available languages
ts-pack download --all

# Download a language group (the manifest currently defines only "all")
ts-pack download --groups all

# Fresh download (clear cache first)
ts-pack download --fresh python

# Check what's cached
ts-pack list --downloaded
```
