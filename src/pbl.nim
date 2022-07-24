import pbl/[types, settings, consts, gen/graphs]
import datamancer

proc generateDataFrame(dataset: DataSet, chunkCount = dsChunkCount, chunkSize = dsChunkSize, minScore: float = DataMinScore): DataFrame =
  var
    retentionTimes, scores: seq[float]
    gravys, gravy2s, gravy3s, masses: seq[float]
    lengths: seq[int]
    counter: int

  for slice in dataset.chunked(chunkCount, chunkSize):
    for entry in slice:
      if entry.hasInvalidSequence():
        when Debug: echo entry.sequence
        continue
      if entry.score < minScore: continue
      counter += 1
      retentionTimes.add entry.retentionTime
      scores.add entry.score
      gravys.add entry.gravy()
      gravy2s.add entry.gravy(HydrophicityScale.hsHoppWoods)
      gravy3s.add entry.gravy(HydrophicityScale.hsCornette)
      masses.add entry.weight()
      lengths.add entry.sequence.len()

  result = toDf({
    "Retention Time": retentionTimes,
    "Score": scores,
    "GRAVY": gravys,
    "GRAVY2": gravy2s,
    "GRAVY3": gravy3s,
    "Mass": masses,
    "Length": lengths
  })
  
  echo counter

when isMainModule:
  initTimer()
  let dsTimer = cpuTime()
  var dataset = loadDataSet()
  echoTime("Loaded: Dataset", since=dsTimer)

  let dfTimer = cpuTime()
  let initialDf = generateDataFrame(dataset)
  echoTime("Generated: Dataframe", since=dfTimer)
  echo "Dataframe size: ", initialDf.len

  let dfSortTimer = cpuTime()
  let scoreSortedDf = initialDf.arrange("Score", order = SortOrder.Ascending)
  echoTime("Sorted: Dataframe (score)", since=dfSortTimer)
  echo scoreSortedDf

  let graphTimer = cpuTime()
  generatePlots(scoreSortedDf, 10000)
  echoTime("Generated: Graphs", since=graphTimer)

  echoTime("Finished")