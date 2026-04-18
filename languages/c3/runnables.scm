; C3 function-level tests: fn ... @test { ... }
((func_definition
  (func_header
    name: (_) @run @test_name)
  (attributes
    (attribute
      name: (at_ident) @_attribute
      (#eq? @_attribute "@test")))) @_
  (#set! tag c3-test))

; C3 module-level tests: module foo @test; then every function in the file is runnable.
((source_file
  (module_declaration
    (attributes
      (attribute
        name: (at_ident) @_module_attribute
        (#eq? @_module_attribute "@test"))))
  (_)*
  (func_definition
    (func_header
      name: (_) @run @test_name)) @_function)
  (#not-match? @_function "@test")
  (#set! tag c3-test))
