interface Config
    exposes [ Config ]
    imports []

# TODO: Make this a `List` and support it in the platform (with `Vec`?)
Config :
    {
        outputFilePath : Str,
        title : Str,
        subtitle : Str,
        width : U32,
        height : U32,
        x : List I32,
        y1 : List I32,
        y2 : List I32,
    }
