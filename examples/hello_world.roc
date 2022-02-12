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
        points1: [[-1, 1], [0, -1], [1, 1]],
        points2: [[-1, -2], [0, 2], [1, -2]],
    }
