app "hello_world"
    packages { pf: "../roc/examples/cli/platform" }
    imports [ pf.Stdout.{ line }, pf.Task.{ await } ]
    provides [ main ] to pf

main = 
    _ <- await (line "")
    line "Hello, World!"
