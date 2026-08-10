```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"hcl\"}")
_ = try TreeSitterLanguagePack.process(source: "resource \"aws_instance\" \"web\" {\n  ami = \"ami-123\"\n  instance_type = \"t2.micro\"\n}\n", config: configObj)

```
