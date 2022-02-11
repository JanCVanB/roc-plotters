platform "roc-plotters/bitmap-chart"
    requires {} { config : Config }
    exposes [ Config ]
    packages {}
    imports [ Config.{ Config } ]
    provides [ configForHost ]

configForHost : Config
configForHost = config
