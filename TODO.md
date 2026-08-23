## To-do

- A codegen proc-macro to quickly load on yaml specifications out-of-tree.
- `.get_<attr>()` methods don't have to internally use the iterator, instead
simply skipping attributes based only on their type number.
- Repeating `Op*` types could probably be de-duplicated using `type A = B;`.
Here `A` can still have its own doc comment.


- Testing (for each sensible netlink family and for parsing primitives).
- Simplify codegen logic.
- Optimize generated code size, e.g. leave out code encoding kernel replies by
default.
- Improve user interface (better error reporting, tooling, etc).
- Benchmarks, fuzzing.
