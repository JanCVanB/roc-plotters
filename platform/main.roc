platform "roc-plotters"
    requires {} { config : Config }
    exposes [ Color, Config ]
    packages {}
    imports [ Config.{ Config } ]
    provides [ configForHost ]

configForHost : Config
configForHost = config
