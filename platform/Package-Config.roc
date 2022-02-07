platform "roc-plotters"
    requires {} { title : Str }
    exposes []
    packages {}
    imports []
    provides [ titleForHost ]

titleForHost : Str
titleForHost = title
