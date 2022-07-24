import nimhdf5
import strutils
import settings

type
  FileOpen* = enum
    foRead
    foWrite
    foReadWrite

  DataSet* = ref object
    len*: int
    file*: H5File
    dSetRetentionTime*: H5DataSet
    dSetScore*: H5DataSet
    dSetSequence*: H5DataSet
    dSetRawFile*: H5DataSet

  DataSlice* = object
    offset*, len*: int
    retentionTimes*: seq[float]
    scores*: seq[float]
    sequences*: seq[string]
    rawFiles*: seq[string]

  DataEntry* = object
    retentionTime*: float
    score*: float
    sequence*: string
    rawFile*: string

proc hasInvalidSequence*(entry: DataEntry): bool =
  entry.sequence.contains("(") or 
  entry.sequence.contains(")")

converter toString*(arr: array[120, char]): string =
  for c in arr:
    if c == '\x00': break
    when RemoveUnderScores: 
      if c == '_': continue
    result = result & c

proc `[]`*(data: DataSet, ids: HSlice[int, int]): DataSlice =
  let offset = ids.a
  let count = ids.b - ids.a
  let sequencesRaw = data.dSetSequence.read_hyperslab(array[120, char], offset = @[offset], count = @[count], full_output = false)
  let rawFilesRaw = data.dSetRawFile.read_hyperslab(array[120, char], offset = @[offset], count = @[count], full_output = false)
  var sequences, rawFiles = newSeq[string](count)

  for i, raw in sequencesRaw: sequences[i] = raw.toString
  for i, raw in rawFilesRaw: rawFiles[i] = raw.toString

  DataSlice(
    len: count,
    offset: offset,
    retentionTimes: data.dSetRetentionTime.read_hyperslab(float, offset = @[offset], count = @[count], full_output = false),
    scores: data.dSetScore.read_hyperslab(float, offset = @[offset], count = @[count], full_output = false),
    sequences: sequences,
    rawFiles: rawFiles
  )

iterator chunked*(data: DataSet, count, times: int, start = 0): DataSlice =
  proc newDataSlice(offset, count: int): DataSlice =
    let sequencesRaw = data.dSetSequence.read_hyperslab(array[120, char], offset = @[offset], count = @[count], full_output = false)
    let rawFilesRaw = data.dSetRawFile.read_hyperslab(array[120, char], offset = @[offset], count = @[count], full_output = false)
    var sequences, rawFiles = newSeq[string](count)

    for i, raw in sequencesRaw: sequences[i] = raw.toString
    for i, raw in rawFilesRaw: rawFiles[i] = raw.toString

    DataSlice(
      len: count,
      offset: offset,
      retentionTimes: data.dSetRetentionTime.read_hyperslab(float, offset = @[offset], count = @[count], full_output = false),
      scores: data.dSetScore.read_hyperslab(float, offset = @[offset], count = @[count], full_output = false),
      sequences: sequences,
      rawFiles: rawFiles
      
    )
  var count = count
  var offset = start
  for i in 0..<times:
    if offset >= data.len: break


    if offset + count >= data.len: 
      count = data.len - offset

    yield newDataSlice(offset, count)
    offset += count


proc `[]`*(slice: DataSlice, idx: int): DataEntry =
    result = DataEntry(
      retentionTime: slice.retentionTimes[idx],
      score: slice.scores[idx],
      sequence: slice.sequences[idx],
      rawFile: slice.rawFiles[idx]
    )

iterator items*(slice: DataSlice): DataEntry =
  for i in 0..<slice.len:
    yield DataEntry(
      retentionTime: slice.retentionTimes[i],
      score: slice.scores[i],
      sequence: slice.sequences[i],
      rawFile: slice.rawFiles[i]
    )

proc openDataFile*(path: string = DataFile, operation: FileOpen = foRead): H5File =
  let op = case operation:
    of foRead: "r" 
    of foWrite: "w" 
    of foReadWrite: "rw"

  result = H5open(path, op)

proc loadDataSet*(path: string = DataFile): DataSet =
  new result
  let file = openDataFile()
  result.file = file
  result.dSetRetentionTime = file["retention_time".dset_str]
  result.dSetScore = file["score".dset_str]
  result.dSetSequence = file["sequence".dset_str]
  result.dSetRawFile = file["raw_file".dset_str]
  result.len = result.dSetRetentionTime.shape[0]

  assert result.dSetRetentionTime.shape[0] == result.dSetScore.shape[0]
  assert result.dSetScore.shape[0] == result.dSetSequence.shape[0]
  assert result.dSetSequence.shape[0] == result.dSetRawFile.shape[0]