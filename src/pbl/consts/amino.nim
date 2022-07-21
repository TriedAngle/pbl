type
  AminoAcid* = enum
    aaA
    aaC
    aaD
    aaE
    aaF
    aaG
    aaH
    aaI
    aaK
    aaL
    aaM
    aaN
    aaP
    aaQ
    aaR
    aaS
    aaT
    aaV
    aaW
    aaY

converter toAminoAcid*(c: char): AminoAcid =
  result = case c:
  of 'A': aaA
  of 'C': aaC
  of 'D': aaD
  of 'E': aaE
  of 'F': aaF
  of 'G': aaG
  of 'H': aaH
  of 'I': aaI
  of 'K': aaK
  of 'L': aaL
  of 'M': aaM
  of 'N': aaN
  of 'P': aaP
  of 'Q': aaQ
  of 'R': aaR
  of 'S': aaS
  of 'T': aaT
  of 'V': aaV
  of 'W': aaW
  of 'Y': aaY
  else: 
    raise newException(ValueError, "invalid character")