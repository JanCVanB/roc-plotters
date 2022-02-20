interface Color
    exposes [
        Color,
        black,
        blue,
        cyan,
        green,
        magenta,
        red,
        white,
        yellow,
    ]
    imports []

Color :
    {
        r: U8,
        g: U8,
        b: U8,
    }

black = { r: 0, g: 0, b: 0 }
blue = { r: 0, g: 0, b: 255 }
cyan = { r: 0, g: 255, b: 255 }
green = { r: 0, g: 255, b: 0 }
magenta = { r: 255, g: 0, b: 255 }
red = { r: 255, g: 0, b: 0 }
white = { r: 255, g: 255, b: 255 }
yellow = { r: 255, g: 255, b: 0 }
