app "math"
    packages { pf: "../platform" }
    imports [ pf.Config ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/math.png",
        title: "Math",
        subtitle: "This is what math looks like.",
        width: 800,
        height: 600,
        lines: [
            { name: "cosine", color: Config.blue, points: cos },
            { name: "cosine x 2", color: Config.red, points: cosX2 },
            { name: "cosine x 3", color: Config.green, points: cosX3 },
            { name: "sine", color: Config.cyan, points: sin },
            { name: "sine x 2", color: Config.blue, points: sinX2 },
            { name: "sine x 3", color: Config.green, points: sinX3 },
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
sinX2 = domain |> List.map (\x -> P2 x (2 * Num.sin x))
cosX3 = domain |> List.map (\x -> P2 x (3 * Num.cos x))
sinX3 = domain |> List.map (\x -> P2 x (3 * Num.sin x))
