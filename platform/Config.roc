interface Config
    exposes [ Config ]
    imports [ Color ]

Bounds :
    {
        xMin: F64,
        xMax: F64,
        yMin: F64,
        yMax: F64,
    }

# TODO: Make this a `List` and support it in the platform (with `Vec`?)
Config :
    {
        outputFilePath : Str,
        title : Str,
        subtitle : Str,
        width : U32,
        height : U32,
        lines : List Line,
        bounds : Bounds,
        fonts : Fonts,
        labels : Labels,
        layout : Layout,
    }

Fonts :
    {
        titleFamily: Str,
        titleSize: U32,
        subtitleFamily: Str,
        subtitleSize: U32,
    }

Labels :
    {
        xCount: Nat,
        yCount: Nat,
    }

Layout :
    {
        chartMargin: U32,
        labelArea: U32,
    }

Line :
    {
        name: Str,
        color: Color.Color,
        points: List [P2 F64 F64],
    }
