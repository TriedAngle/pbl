import ../types, ../settings, ../consts, ../utils
import datamancer, zero_functional, plotly
import os

proc plotGravyRetention(xs: seq[float], ys: seq[float], scores: seq[float]) =
  var sizes: seq[float]
  var colors: seq[float]

  for score in scores:
    colors.add score
    sizes.add map(score, DataMinScore, DataMaxScore, PlotGravyRetentionMinSize, PlotGravyRetentionMaxSize)

  var d = Trace[float64](
    mode: PlotMode.Markers, `type`: PlotType.ScatterGL,
    xs: xs, ys: ys, 
    name: "Score"
  )

  d.marker = Marker[float64](size: sizes, colorVals: colors, colorMap: ColorMap.Viridis)

  var layout = Layout(
    title: "GRAVY vs. Retention Time", width: 1280, height: 720,
    xaxis: Axis(title: "GRAVY"),
    yaxis: Axis(title: "Retention Time"),
    font: Font(size: 26)
  )

  var p = Plot[float64](layout: layout, traces: @[d])
  # p.show()
  p.show(filename = PlotsDir / "gravy-retentiontime" & endingName() & ".png")


proc plotMassRetention(xs: seq[float], ys: seq[float], scores: seq[float]) =
  var sizes: seq[float]
  var colors: seq[float]

  for score in scores:
    colors.add score
    sizes.add map(score, DataMinScore, DataMaxScore, PlotGravyRetentionMinSize, PlotGravyRetentionMaxSize)

  var d = Trace[float64](
    mode: PlotMode.Markers, `type`: PlotType.ScatterGL,
    xs: xs, ys: ys, 
    name: "Score"
  )

  d.marker = Marker[float64](size: sizes, colorVals: colors, colorMap: ColorMap.Viridis)

  var layout = Layout(
    title: "Mass vs. Retention Time", width: 1280, height: 720,
    xaxis: Axis(title: "Mass"),
    yaxis: Axis(title: "Retention Time"),
    font: Font(size: 26)
  )

  var p = Plot[float64](layout: layout, traces: @[d])
  # p.show()
  p.show(filename = PlotsDir / "mass-retentiontime" & endingName() & ".png")


proc plotLengthRetention(xs: seq[float], ys: seq[float], scores: seq[float]) =
  var sizes: seq[float]
  var colors: seq[float]

  for score in scores:
    colors.add score
    sizes.add map(score, DataMinScore, DataMaxScore, PlotGravyRetentionMinSize, PlotGravyRetentionMaxSize)

  var d = Trace[float64](
    mode: PlotMode.Markers, `type`: PlotType.ScatterGL,
    xs: xs, ys: ys, 
    name: "Score"
  )

  d.marker = Marker[float64](size: sizes, colorVals: colors, colorMap: ColorMap.Viridis)

  var layout = Layout(
    title: "Length vs. Retention Time", width: 1280, height: 720,
    xaxis: Axis(title: "Length"),
    yaxis: Axis(title: "Retention Time"),
    font: Font(size: 26),
  )

  var p = Plot[float64](layout: layout, traces: @[d])
  # p.show()
  p.show(filename = PlotsDir / "length-retentiontime" & endingName() & ".png")


# nim-plotly is a little sussy here, so I just print the values and write them in a python script in sripts/dirtyheatmap.py lmao
# proc plotHeatMap(retentionTime, gravy, gravy2, gravy3, mass, length: seq[float]) =
#   let corrRetentionRetention = corr(retentionTime, retentionTime)
#   let corrRetentionGravy = corr(retentionTime, gravy)
#   let corrRetentionMass = corr(retentionTime, mass)
#   let corrRetentionLength = corr(retentionTime, length)
#   let corrGravyGravy = corr(gravy, gravy)
#   let corrGravyMass = corr(gravy, mass)
#   let corrGravyLength = corr(gravy, length)
#   let corrMassMass = corr(mass, mass)
#   let corrMassLength = corr(mass, length)
#   let corrLengthLength = corr(length, length)

#   let corrGravyGravy2 = corr(gravy, gravy2)
#   let corrGravyGravy3 = corr(gravy, gravy3)
#   let corrGravy2Gravy3 = corr(gravy2, gravy3)

#   echo "rr ", corrRetentionRetention
#   echo "rg ", corrRetentionGravy
#   echo "rm ", corrRetentionMass
#   echo "rl ", corrRetentionLength
#   echo "gg ", corrGravyGravy
#   echo "gm ", corrGravyMass
#   echo "gl ", corrGravyLength
#   echo "mm ", corrMassMass
#   echo "ml ", corrMassLength
#   echo "ll ", corrLengthLength 
#   echo "--- "

#   echo "gg2  ", corrGravyGravy2
#   echo "gg3  ", corrGravyGravy3
#   echo "g2g3 ", corrGravy2Gravy3

  # var d = Trace[float](
  #   mode: PlotMode.Lines, `type`: PlotType.HeatMap,
  #   colormap: ColorMap.Viridis,
  #   zs: corrMatrix, xs: xs
  # )

  # let layout = Layout(
  #   title: "Heatmap test", width: 1280, height: 720,
  # )

  # var p = Plot[float](layout: layout, traces: @[d])
  # p.show(filename = PlotsDir / "heatmap.png")



proc generatePlots*(dataFrame: DataFrame, chunkSize = dsChunkSize) =
  var
    retentionTimes, scores: seq[float]
    gravys, gravy2s, gravy3s, masses: seq[float] 
    lengths: seq[float]
  
  for row in dataFrame:
    retentionTimes.add row["Retention Time"].fnum
    scores.add row["Score"].fnum
    gravys.add row["GRAVY"].fnum
    gravy2s.add row["GRAVY2"].fnum
    gravy3s.add row["GRAVY3"].fnum
    masses.add row["Mass"].fnum
    lengths.add row["Length"].num.toFloat

  plotGravyRetention(gravys, retentionTimes, scores)
  plotMassRetention(masses, retentionTimes, scores)
  plotLengthRetention(lengths, retentionTimes, scores)
  # plotHeatMap(retentionTimes, gravys, gravy2s, gravy3s, masses, lengths)