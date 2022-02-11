platform "roc-plotters/bitmap-chart"
    requires {} { config : configForHost }
    exposes []
    packages {}
    imports []
    provides [ configForHost ]

configForHost : {
    outputFilePath : Str,
    title : Str,
    subtitle : Str,
    width : U32,
    height : U32,
    }
configForHost = config
