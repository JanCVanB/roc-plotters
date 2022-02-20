app "hello_world"
    packages { pf: "../platform" }
    imports [ pf.Config ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.png",
        title: "Hello, World!",
        subtitle: "",
        width: 800,
        height: 600,
        lines: [
            { name: "up", color: Config.green, points: [P2 -1 -1, P2 1 1] },
            { name: "down", color: Config.red, points: [P2 -1 1, P2 1 -1] },
        ],
        bounds: {
            xMin: -1.1,
            xMax: 1.1,
            yMin: -1.1,
            yMax: 1.1,
        },
        labels: {
            xCount: 3,
            yCount: 3,
        },
        layout: {
            chartMargin: 5,
            labelArea: 50,
        },
        fonts: {
            titleFamily: "sans-serif",
            titleSize: 60,
            subtitleFamily: "sans-serif",
            subtitleSize: 40,
        },
    }
