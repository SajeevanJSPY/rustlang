# RustLang

Creating Front-end for the Rust Language
[lexical grammar reference](https://doc.rust-lang.org/reference/tokens.html)
[rust frontend (rustc)](https://github.com/rust-lang/rust/tree/master/compiler)
[c/c++ frontend (clang)](https://github.com/llvm/llvm-project/tree/main/clang)
[compiler backend](https://llvm.org/docs)


## Simple TODO  
                    SOURCE File (*.rs)
                            |
                            | Lexing and Parsing
                            |
                  AST (Abstract Syntax Tree)
                            |
                            | IR Code Generation
                            |
                 Intermediate Representation (IR)
                            |
                            | LLVM
                            |
           Target Specific Assembly Code Or Object File 

This Repository is not about creating a production-ready Rust compiler.
Try to create compiler infrastructure as a beginner.
Using **LLVM** for the Backend of the compiler 

