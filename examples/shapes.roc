app "shapes"
    packages { pf: "../platform" }
    imports [ pf.Config ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/shapes.jpg",
        title: "Shapes",
        subtitle: "",
        width: 800,
        height: 600,
        lines: [
            {
                name: "rectangle",
                color: Config.red,
                points: [
                    P2 0.1 0.3,
                    P2 0.2 0.3,
                    P2 0.2 0.2,
                    P2 0.1 0.2,
                    P2 0.1 0.3,
                ],
            },
            {
                name: "triangle",
                color: Config.green,
                points: [
                    P2 0.4 0.6,
                    P2 0.5 1.0,
                    P2 0.6 0.6,
                    P2 0.4 0.6,
                ],
            },
            {
                name: "plus",
                color: Config.blue,
                points: [
                    P2 0.6 0.3, P2 0.8 0.3,
                    P2 0.7 0.3,
                    P2 0.7 0.4, P2 0.7 0.2,
                ],
            },
        ],
        bounds: {
            xMin: 0,
            xMax: 1,
            yMin: 0,
            yMax: 1,
        },
        labels: {
            xCount: 5,
            yCount: 5,
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
