# cesrinfo

[CESR](https://trustoverip.github.io/tswg-cesr-specification/)


## [Parse part sizes](https://trustoverip.github.io/tswg-cesr-specification/#table-4)

Label	Description
‘hs’	hard size in chars (fixed) part of code size
‘ss’	soft size in chars, (Variable) part of code size
‘os’	other size in chars, when soft part includes two Variable values
‘ms’	main size in chars, (derived) when soft part provides two Variable values where ms = ss – os
‘cs’	code size in chars (derived value), where cs = hs + ss
‘vs’	value size in chars
‘fs’	full size in chars where fs = hs + ss + vs
‘ls’	lead size in bytes to pre-pad raw binary bytes
‘ps’	pad size in chars Base64 encoded
‘rs’	raw size in bytes (derived) of binary value where rs is derived from R(T)
‘bs’	binary size in bytes (derived) where bs = ls + rs

