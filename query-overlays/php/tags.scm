; Class definitions
(class_declaration name: (name) @name) @definition.class

; Interface definitions
(interface_declaration name: (name) @name) @definition.interface

; Method definitions
(method_declaration name: (name) @name) @definition.method

; Function definitions
(function_definition name: (name) @name) @definition.function

; Class inheritance: `class Foo extends Bar`
(class_declaration
  (base_clause (name) @name)) @reference.implementation

; Interface implementation: `class Foo implements Bar, Baz`
(class_declaration
  (class_interface_clause (name) @name)) @reference.implementation

; Interface extension: `interface Foo extends Bar`
(interface_declaration
  (base_clause (name) @name)) @reference.implementation

; Function / method calls
(function_call_expression function: (name) @name) @reference.call
; ~keep: method_call_expression does not exist in this grammar pin; member_call_expression
; is the equivalent node (see parsers/php/src/node-types.json and the vendored
; parsers/php/queries/tags.scm, which uses the same node type).
(member_call_expression name: (name) @name) @reference.call
