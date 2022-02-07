platform "roc-plotters"
    requires {} {
        options : {
            outputFilePath : Str,
            title : Str,
        }
    }
    exposes []
    packages {}
    imports []
    provides [ optionsForHost ]

optionsForHost : {
    outputFilePath : Str,
    title : Str,
    }
optionsForHost = options
