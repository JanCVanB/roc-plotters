interface Config
    exposes [ Config, black, white, red, green, blue, cyan, magenta, yellow ]
    imports []

Bounds :
    {
        xMin: F64,
        xMax: F64,
        yMin: F64,
        yMax: F64,
    }

# TODO: probably move some of this into a different module.
Color :
    {
        r: U8,
        g: U8,
        b: U8,
    }

black = { r: 0, g: 0, b: 0 }
white = { r: 255, g: 255, b: 255 }
red = { r: 255, g: 0, b: 0 }
green = { r: 0, g: 255, b: 0 }
blue = { r: 0, g: 0, b: 255 }
cyan = { r: 0, g: 255, b: 255 }
magenta = { r: 255, g: 0, b: 255 }
yellow = { r: 255, g: 255, b: 0 }

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
        color: Color,
        points: List [P2 F64 F64],
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
