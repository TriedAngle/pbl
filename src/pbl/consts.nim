import zero_functional
import types, settings
import consts/[amino, hydrophobicity, weight]
export amino, hydrophobicity, weight

proc calcGRAVY*(sequence: string, kind: HydrophicityScale = hsKyteDoolittle): float = 
  let calcFn = case kind:
    of hsKyteDoolittle: toKyteDoolittle
    else: quit($kind & "not implemented yet")
      
  
  let sum = sequence --> map(it.calcFn()).sum()

  when RemoveUnderScores:
    result = sum / sequence.len.float
  else: 
    result = sum / (sequence.len - 2).float

proc gravy*(entry: DataEntry, kind: HydrophicityScale = hsKyteDoolittle): float =
  entry.sequence.calcGRAVY(kind)


proc calcMolecularWeight*(sequence: string): float = 
  result = sequence --> map(it.toMolecularWeight()).sum() + NTermH + CTermOH


proc weight*(entry: DataEntry): float =
  entry.sequence.calcMolecularWeight()