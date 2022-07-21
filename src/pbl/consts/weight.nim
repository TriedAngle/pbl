# Values taken from:
# https://pepcalc.com/notes.php?mw
import amino

const NTermH*  =  1.00797
const CTermOH* = 17.00738

const MolecularWeight* = [
    aaA: 71.07793,
    aaC: 103.1454,
    aaD: 115.0873,
    aaE: 129.1139,
    aaF: 147.1734,
    aaG: 57.05138,
    aaH: 137.1394,
    aaI: 113.1576,
    aaK: 128.1724,
    aaL: 113.1576,
    aaM: 131.1985,
    aaN: 114.1028,
    aaP: 97.11508,
    aaQ: 128.1293,
    aaR: 156.1861,
    aaS: 87.07733,
    aaT: 101.1039,
    aaV: 99.13103,
    aaW: 186.2095,
    aaY: 163.1728
]

converter toMolecularWeight*(c: char): float =
  result = case c:
  of 'A': 71.07793
  of 'C': 103.1454
  of 'D': 115.0873
  of 'E': 129.1139
  of 'F': 147.1734
  of 'G': 57.05138
  of 'H': 137.1394
  of 'I': 113.1576
  of 'K': 128.1724
  of 'L': 113.1576
  of 'M': 131.1985
  of 'N': 114.1028
  of 'P': 97.11508
  of 'Q': 128.1293
  of 'R': 156.1861
  of 'S': 87.07733
  of 'T': 101.1039
  of 'V': 99.13103
  of 'W': 186.2095
  of 'Y': 163.1728
  of '_': 0.0  
  else: 
    raise newException(ValueError, "invalid character")