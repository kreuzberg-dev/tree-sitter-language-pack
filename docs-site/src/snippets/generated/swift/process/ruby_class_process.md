```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ruby\"}")
_ = try TreeSitterLanguagePack.process(source: "require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello #{name}\"\n  end\nend\n", config: configObj)

```
