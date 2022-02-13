app "hello_world"
    packages { pf: "../platform" }
    imports []
    provides [ config ] to pf

config =
    {
        outputFilePath: "./examples/hello_world.svg",
        title: "Hello, World!",
        subtitle: "This data is coming from Roc ☺",
        width: 1024,
        height: 768,
        lines: [sin, cos, sin2, cos2],
    }

pi = 3.141592653589793
ok = \r -> Result.withDefault r 0
domain = List.range -100 101 |> List.map (\i -> pi * (Num.toFloat i) / 100 |> ok)
cos = domain |> List.map (\x -> P2 x (Num.cos x))
sin = domain |> List.map (\x -> P2 x (Num.sin x))
cos2 = domain |> List.map (\x -> P2 x (2 * Num.cos x))
sin2 = domain |> List.map (\x -> P2 x (2 * Num.sin x))
