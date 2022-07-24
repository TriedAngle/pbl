# Package

version       = "0.1.0"
author        = "Sebastian Strobl"
description   = "A new awesome nimble package"
license       = "Apache-2.0"
srcDir        = "src"
bin           = @["pbl"]


# Dependencies

requires "nim >= 1.6.6"
requires "zero_functional"
requires "nimhdf5"
requires "plotly"
requires "chroma"
requires "weave"
requires "datamancer"
requires "arraymancer"
requires "ggplotnim"