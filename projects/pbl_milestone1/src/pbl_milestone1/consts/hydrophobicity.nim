# Values taken from:
# https://resources.qiagenbioinformatics.com/manuals/clcgenomicsworkbench/650/Hydrophobicity_scales.html

import amino

type
  HydrophicityScale* = enum
    hsKyteDoolittle
    hsHoppWoods
    hsCornette
    hsEisenberg
    hsRose
    hsJanin
    hsEngelmanGES

const KyteDoolittle* = [
    aaA:  1.80,
    aaC:  2.50,
    aaD: -3.50,
    aaE: -3.50,
    aaF:  2.80,
    aaG: -0.40,
    aaH: -3.20,
    aaI:  4.50,
    aaK: -3.90,
    aaL:  3.80,
    aaM:  1.90,
    aaN: -3.50,
    aaP: -1.60,
    aaQ: -3.50,
    aaR: -4.50,
    aaS: -0.80,
    aaT: -0.70,
    aaV:  4.20,
    aaW: -0.90,
    aaY: -1.30
]

proc toKyteDoolittle*(c: char): float =
  result = case c:
  of 'A':  1.80 
  of 'C':  2.50 
  of 'D': -3.50
  of 'E': -3.50
  of 'F':  2.80 
  of 'G': -0.40
  of 'H': -3.20
  of 'I':  4.50 
  of 'K': -3.90
  of 'L':  3.80 
  of 'M':  1.90 
  of 'N': -3.50
  of 'P': -1.60
  of 'Q': -3.50
  of 'R': -4.50
  of 'S': -0.80
  of 'T': -0.70
  of 'V':  4.20 
  of 'W': -0.90
  of 'Y': -1.30
  of '_':  0.0
  else: 
    raise newException(ValueError, "Character: '" & $c & "' invalid")

proc toHoppWoods*(c: char): float =
  result = case c:
  of 'A': -0.50
  of 'C': -1.00
  of 'D':  3.00
  of 'E':  3.00
  of 'F': -2.50
  of 'G':  0.00
  of 'H': -0.50
  of 'I': -1.80
  of 'K':  3.00
  of 'L': -1.80
  of 'M': -1.30
  of 'N':  0.20
  of 'P':  0.00
  of 'Q':  0.20
  of 'R':  3.00
  of 'S':  0.30
  of 'T': -0.40
  of 'V': -1.50
  of 'W': -3.40
  of 'Y': -2.30
  of '_': 0.0
  else: 
    raise newException(ValueError, "Character: '" & $c & "' invalid")

proc toCornette*(c: char): float =
  result = case c:
  of 'A':  0.20
  of 'C':  4.10 
  of 'D': -3.10
  of 'E': -1.80
  of 'F':  4.40 
  of 'G':  0.00
  of 'H':  0.50
  of 'I':  4.80 
  of 'K': -3.10
  of 'L':  5.70 
  of 'M':  4.20 
  of 'N': -0.50
  of 'P': -2.20
  of 'Q': -2.80
  of 'R':  1.40
  of 'S': -0.50
  of 'T': -1.90
  of 'V':  4.70 
  of 'W':  1.00
  of 'Y':  3.20
  of '_':  0.0
  else: 
    raise newException(ValueError, "Character: '" & $c & "' invalid")