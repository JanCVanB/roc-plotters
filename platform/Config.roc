interface Config
    exposes [ Config, colors ]
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
        color: Color,
        points: List [P2 F64 F64],
    }

black : Color
black = { r: 0, g: 0, b: 0 }
blue : Color
blue = { r: 0, g: 0, b: 255 }
cyan : Color
cyan = { r: 0, g: 255, b: 255 }
green : Color
green = { r: 0, g: 255, b: 0 }
magenta : Color
magenta = { r: 255, g: 0, b: 255 }
red : Color
red = { r: 255, g: 0, b: 0 }
white : Color
white = { r: 255, g: 255, b: 255 }
yellow : Color
yellow = { r: 255, g: 255, b: 0 }

colors : { black : Color, blue : Color, cyan : Color, green : Color, magenta : Color, red : Color, white : Color, yellow : Color }
colors = { black, blue, cyan, green, magenta, red, white, yellow }
