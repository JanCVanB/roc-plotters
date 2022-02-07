platform "roc-plotters"
    requires {} {
        options : {
            outputFilePath : Str,
            subtitle : Str,
            title : Str,
        }
    }
    exposes []
    packages {}
    imports []
    provides [ optionsForHost ]

optionsForHost : {
    outputFilePath : Str,
    subtitle : Str,
    title : Str,
    }
optionsForHost = options
