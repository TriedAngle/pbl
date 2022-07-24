import zero_functional, math

proc map*(val, inStart, inEnd, outStart, outEnd: float): float =
  let slope = (outEnd - outStart) / (inEnd - inStart)
  result = outStart + slope * (val - inStart)

proc corr*(seqA, seqB: seq[float]): float =
  assert seqA.len() == seqB.len()
  let n = seqA.len().toFloat
  let a = seqA.sum()
  let b = seqB.sum()
  let ab = zip(seqA, seqB) --> map(it[0] * it[1]).sum()
  let a2 = seqA --> map(it * it).sum()
  let b2 = seqB --> map(it * it).sum()

  let top = (n * ab) - (a * b)
  let bottom = (n * a2 - (a * a)) * (n * b2 - (b * b))
  result =  top / sqrt(bottom)