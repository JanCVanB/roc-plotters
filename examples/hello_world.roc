app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "This data is coming from Roc:",
        width: 1024,
        height: 768,
        x: [-1, 0, 1],
        y1: [1, -1, 1],
        y2: [-2, 2, -2],
    }
