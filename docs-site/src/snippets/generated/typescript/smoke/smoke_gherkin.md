```typescript title="TypeScript"
import { process } from "@xberg-io/tree-sitter-language-pack";
function main() {
  const result = process("Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n", { language: "gherkin" });
}

void main();

```
