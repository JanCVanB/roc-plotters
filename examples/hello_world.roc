app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ options ] to pf

options = {
    outputFilePath: "./examples/hello_world.png",
    title: "Hello, World!",
    subtitle: "These strings are coming from Roc :)",
    width: 1024,
    height: 768,
    }
