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
        points1: [P2 -1  1, P2 0 -1, P2 1 1],
        points2: [P2 -1 -2, P2 0  2, P2 1 -2],
    }
