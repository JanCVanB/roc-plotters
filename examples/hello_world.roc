app "hello_world"
    packages { pf: "../platform" }
    imports [ pf.Config ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "",
        width: 1024,
        height: 768,
        lines: [
            { name: "cosine", color: Config.green, points: cos },
            { name: "cosine x 2", color: Config.cyan, points: cosX2 },
            { name: "sine", color: Config.blue, points: sin },
            { name: "sine x 2", color: Config.red, points: sinX2 },
        ],
        bounds: {
            xMin: -3.2,
            xMax: 3.2,
            yMin: -2.1,
            yMax: 2.1,
        },
        fonts: {
            titleFamily: "sans-serif",
            titleSize: 60,
            subtitleFamily: "sans-serif",
            subtitleSize: 40,
        },
        labels: {
            xCount: 20,
            yCount: 10,
        },
        layout: {
            chartMargin: 5,
            labelArea: 50,
        },
    }

pi = 3.141592653589793
ok = \r -> Result.withDefault r 0
domain = List.range -100 101 |> List.map (\i -> pi * (Num.toFloat i) / 100 |> ok)
cos = domain |> List.map (\x -> P2 x (Num.cos x))
sin = domain |> List.map (\x -> P2 x (Num.sin x))
cosX2 = domain |> List.map (\x -> P2 x (2 * Num.cos x))
sinX2 = domain |> List.map (\x -> P2 x (2 * Num.sin x))
