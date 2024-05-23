# 8-colour Palettes
These are probably the most used ANSI escape codes, used for basic colouring and are often customized by the user.

They fit in the following SGR ranges:
30-37: Foreground regular
40-47: Background regular
90-97: Foreground bright
100-107: Background bright
Put together, an example would look like `ESC[31m`

They are in the following order: red, green, yellow, blue, purple and cyan.
They are ordered in what was considered most common to show up; red was first for errors, green for successes, yellow for warnings, and blue, purple and cyan all for assorted other reasons.

# 255-colour Palettes
Generally the least used colour codes, mostly being overtaken by 24-bit colour.
`ESC[38;5;{code}m` produces them, with code being an integer between 0-255.

# 24-bit Colour, AKA Full Colour
Commonly used in editors such as Vim or Emacs, these allow the terminal to render text in any colour that most modern screens support.
`ESC[38;2;{r};{g};{b}m`

# Other Colour Codes
There are also a handful of other ANSI codes that can be used to change colours, but most are lesser supported, and often times are rebrands of existing commands and should not be considered.

