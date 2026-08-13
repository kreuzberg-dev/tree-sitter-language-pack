; Class definitions
(class_definition name: (identifier) @name) @definition.class

; Method declarations inside class bodies
; ~keep: method_signature has no fields of its own — it wraps a function/getter/setter
; signature, and only the wrapped node carries `name`. `method_signature name:` is an
; impossible pattern that tree-sitter rejects at compile time.
(method_signature
  (function_signature name: (identifier) @name)) @definition.method
(method_signature
  (getter_signature name: (identifier) @name)) @definition.method
(method_signature
  (setter_signature name: (identifier) @name)) @definition.method
(function_signature name: (identifier) @name) @definition.function

; Class superclass: `class Foo extends Bar`
(class_definition
  superclass: (superclass (type_identifier) @name)) @reference.implementation

; Class interfaces: `class Foo implements Bar, Baz`
(class_definition
  interfaces: (interfaces (type_identifier) @name)) @reference.implementation

; Function / method calls
; ~keep: function_expression_invocation does not exist in this grammar pin;
; calls are structurally an (identifier) followed by a (selector (argument_part ...)),
; matching the pattern used by the vendored parsers/dart/queries/tags.scm.
((identifier) @name
 (selector
    "!"?
    (conditional_assignable_selector
      "?." (identifier) @name)?
    (unconditional_assignable_selector
      "."? (identifier) @name)?
    (argument_part
      (arguments
        (argument)*))?)*
	(cascade_section
	  (cascade_selector
		(identifier)) @name
	  (argument_part
		(arguments
		  (argument)*))?)?) @reference.call
