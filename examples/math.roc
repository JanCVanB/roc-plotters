app "math"
    packages { pf: "../platform" }
    imports [ pf.Color ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/math.svg",
        title: "Math",
        subtitle: "This is what math looks like.",
        width: 800,
        height: 600,
        lines: [
            {
                name: "cosine",
                color: Color.green,
                pointRadius: 0,
                points: cos,
                isLineVisible: True,
                isPointVisible: False,
            },
            {
                name: "cosine x 2",
                color: Color.cyan,
                pointRadius: 0,
                points: cosX2,
                isLineVisible: True,
                isPointVisible: False,
            },
            {
                name: "sine",
                color: Color.blue,
                pointRadius: 0,
                points: sin,
                isLineVisible: True,
                isPointVisible: False,
            },
            {
                name: "- sine",
                color: Color.red,
                pointRadius: 0,
                points: sinNeg,
                isLineVisible: True,
                isPointVisible: False,
            },
        ],
        bounds: {
            xMin: -3.2,
            xMax: 3.2,
            yMin: -2.1,
            yMax: 2.1,
        },
        labels: {
            xCount: 20,
            yCount: 10,
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

pi = 3.141592653589793
ok = \r -> Result.withDefault r 0
domain = List.range -100 101 |> List.map (\i -> pi * (Num.toFloat i) / 100 |> ok)
cos = domain |> List.map (\x -> P2 x (Num.cos x))
sin = domain |> List.map (\x -> P2 x (Num.sin x))
cosX2 = domain |> List.map (\x -> P2 x (2 * Num.cos x))
sinNeg = domain |> List.map (\x -> P2 x (0 - Num.sin x))
