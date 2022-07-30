import os, times
export times

const 
  Debug* = false
  BaseDir = currentSourcePath.parentDir()
  AssetDir* = BaseDir / "../../assets"
  PlotsDir* = AssetDir / "plots" 
  DataSetsDir* = AssetDir / "datasets"
  DataFile* = DataSetsDir / "retention_time.hdf5"

  RemoveUnderScores* = true

  DataMaxScore* = 574.97

  DataMinScore* = 100


  dsChunkCount* = 1000
  dsChunkSize* = 10

  PlotGravyRetentionMinSize* = 5.0
  PlotGravyRetentionMaxSize* = 25.0

var 
  timer*: float

proc initTimer*() = timer = cpuTime()

proc timeSinceStart*(): float = cpuTime() - timer

proc echoTime*(msg: string, now = cpuTime(), since = timer) =
  echo msg, " - took: ", now - since, "s"


proc endingName*(): string =
  "min" & $ DataMinScore & "count" & $dsChunkCount & "size" & $dsChunkSize & 
  "ssmin" & $ PlotGravyRetentionMinSize & "ssmax" & $PlotGravyRetentionMaxSize