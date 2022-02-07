platform "roc-plotters"
    requires {} {
        options : {
            outputFilePath : Str,
            title : Str,
            subtitle : Str,
            width : U32,
            height : U32,
        }
    }
    exposes []
    packages {}
    imports []
    provides [ optionsForHost ]

optionsForHost : {
    outputFilePath : Str,
    title : Str,
    subtitle : Str,
    width : U32,
    height : U32,
    }
optionsForHost = options
