
Mango IR (intermediary representation)
===============================

Mango IR is the data structure that represents the code after parsing and checking, but before code generation and most optimizations.

This data is produced by the `compiler frontend`_, and compiled to WebAssembly by a backend, or interpreted by the interpreter_.

Note that, for the time being, the intermediary format has no stability guarantees.

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

Links
-------------------------------

* `Official website`_
* `Documentation`_
* `Code of conduct and contributing`_


.. _`Official website`: https://mangocode.org/
.. _`Documentation`: https://docs.mangocode.org/
.. _`Code of conduct and contributing`: https://github.com/mangolang/mango
.. _`compiler frontend`: https://github.com/mangolang/compiler
.. _interpreter: https://github.com/mangolang/interpreter
