```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "# A comment\ndef greet(name):\n    \"\"\"Say hello.\"\"\"\n    return f'Hi {name}'\n\nimport os\n", config: configObj)

```
