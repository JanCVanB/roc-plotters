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
        points1 : List ([P2 I32 I32]),
        points2 : List ([P2 I32 I32]),
    }
