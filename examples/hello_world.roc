app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ options ] to pf

options = {
    title: "Hello, World!",
    outputFilePath: "./examples/hello_world.png",
    }
