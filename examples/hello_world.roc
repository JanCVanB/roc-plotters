app "hello_world"
    packages { pf: "../platform" }
    imports [ pf.Config.{blue, green, red, cyan} ]
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "This data is coming from Roc ☺",
        width: 1024,
        height: 768,
        lines: [
            { name: "sine", color: blue, points: sin },
            { name: "cosine", color: green, points: cos },
            { name: "sine2", color: red, points: sin2 },
            { name: "cosine2", color: cyan, points: cos2 },
        ],
    }

pi = 3.141592653589793
ok = \r -> Result.withDefault r 0
domain = List.range -100 101 |> List.map (\i -> pi * (Num.toFloat i) / 100 |> ok)
cos = domain |> List.map (\x -> P2 x (Num.cos x))
sin = domain |> List.map (\x -> P2 x (Num.sin x))
cos2 = domain |> List.map (\x -> P2 x (2 * Num.cos x))
sin2 = domain |> List.map (\x -> P2 x (2 * Num.sin x))
