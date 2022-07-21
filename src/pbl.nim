import pbl/[types, settings, consts]
import zero_functional, plotly
import os, times

const
  minScore = 400.0
  maxScore = 574.97

  minScoreSize = 10.0
  maxScoreSize = 40.0
  

proc map*(val, inStart, inEnd, outStart, outEnd: float): float =
  let slope = (outEnd - outStart) / (inEnd - inStart)
  result = outStart + slope * (val - inStart)

when isMainModule:
  var dataset = loadDataSet()
  let start = cpuTime()
  
  var 
    x, y: seq[float64]
    colors, sizes: seq[float64]
    scores: seq[string]

  for slice in dataset.chunked(50000, 1000):
    for entry in slice:
      if entry.hasInvalidSequence(): continue
      if entry.score < minScore: continue
      colors.add entry.score
      sizes.add map(entry.score, minScore, maxScore, minScoreSize, maxScoreSize)

      x.add entry.gravy()
      y.add entry.retentionTime

      scores.add $entry.score


  var d = Trace[float64](
    mode: PlotMode.Markers, `type`: PlotType.ScatterGL,
    xs: x, ys: y, text: scores
  )
  d.marker = Marker[float64](size: sizes, colorVals: colors, colorMap: ColorMap.Viridis)

  var layout = Layout(
    title: "GRAVY vs. Retention Time", width: 1280, height: 720,
    xaxis: Axis(title: "GRAVY"),
    yaxis: Axis(title: "Retention Time")
  )

  var p = Plot[float64](layout: layout, traces: @[d])

  echo cpuTime() - start
  p.show()