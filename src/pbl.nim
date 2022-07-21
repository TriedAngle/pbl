import pbl/[types, settings, consts]
import zero_functional, plotly
import os, times

const maxScore = 574.97

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

  for slice in dataset.chunked(5000, 1000):
    for entry in slice:
      if entry.hasInvalidSequence(): continue
      if entry.score < 200: continue
      colors.add map(entry.score, 0.0, maxScore, -1.0, 1.0)
      sizes.add map(entry.score, 0.0, maxScore, 0.5, 24.0)

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