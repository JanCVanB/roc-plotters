app "hello_world"
    packages { pf: "../platforms/bitmap-chart" }
    imports []
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "These strings are coming from Roc :)",
        width: 1024,
        height: 768,
    }
