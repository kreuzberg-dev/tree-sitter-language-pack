```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print('hello')\n\ndef world():\n    print('world')\n", config: configObj)

```
