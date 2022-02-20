app "scatter"
    packages { pf: "../platform" }
    imports [ pf.Color, ScatterRandom ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/scatter.png",
        title: "Scatterplot of pseudo-random points",
        subtitle: "",
        width: 1600,
        height: 1200,
        lines: [
            {
                name: "pseudo-random points",
                color: Color.blue,
                pointRadius: 0,
                points: randomPoints,
                isLineVisible: False,
                isPointVisible: True,
            },
        ],
        bounds: {
            xMin: -10000,
            xMax: 110000,
            yMin: -10000,
            yMax: 110000,
        },
        labels: {
            xCount: 2,
            yCount: 2,
        },
        layout: {
            chartMargin: 10,
            labelArea: 100,
        },
        fonts: {
            titleFamily: "sans-serif",
            titleSize: 100,
            subtitleFamily: "sans-serif",
            subtitleSize: 80,
        },
    }

pointCount = 100000
randomSeed = 123
min = 0
max = 100000
randomPoints = List.repeat pointCount 0
    |> List.walk { previous: { seed: ScatterRandom.seed32 randomSeed, value: randomSeed }, values: [] } (\state, _ ->
            now = ScatterRandom.step state.previous.seed (ScatterRandom.u32 min max)
            x = Num.toFloat state.previous.value
            y = Num.toFloat now.value
            values = List.append state.values (P2 x y)
            { previous: now, values }
        )
    |> \w -> w.values
