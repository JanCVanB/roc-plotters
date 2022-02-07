app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ title ] to pf

title = "Hello, World!"
