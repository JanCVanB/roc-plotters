app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ options ] to pf

options = {
    outputFilePath: "./examples/hello_world.png",
    subtitle: "These strings are coming from Roc :)",
    title: "Hello, World!",
    }
