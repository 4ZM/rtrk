# The Rusty TRacKer

A console based Synth and Tracker for making sweet noise

```
┏━━━━━━━━━[ rtrk ]━━━━━━━━━━━━━━━━━━━━━━━━━━ , ━━━━━━ [v0.1] ━━━ =^..^= ━━━━━━━┓
┃                                     ______/ \_ _/\______,___/\ ___' _____,   ┃
┃ >01 . 1 AADDSSRR LLHHXX             \         \   ____/       \   :/    /    ┃
┃  02 : 4 -------- ------             /    <    /:  \ \    >    /   ;   _/     ┃
┃  03 : - -------- ------            /         < |   \/       <<         \     ┃
┃  04 : - -------- ------           /      :    \|    \    ;    \   ,     \    ┃
┃  05 : - -------- ------           \      |     \    /    |     \  :      \   ┃
┃  06 ' - -------- ------            \  ___^_____/   /\____|     /__:       \  ┃
┃                                     \/   ;      \ /  4ZM  \___/   |_______/  ┃
┠──────────────────────────────────────────────────'───────────────────────────┨
┃ ▚▚▚▚▚▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚▞▚█████                            | << | . |[>]|        ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ ## : ▁▂▃▄▅▆▇██▆▅▃  :  ▆▅▃▁▂▃▄▅▆▇█   :  ▅▆▇█▅▆▇█▆▅▃▁  : ▃▁▁▁▂▃▄▅▆▇█▆▃  :  gFx ┃
┠──────────────────────────────────────────────────────────────────────────────┨
┃ 09 . C#4 1 A0 101  .  --- - -- ---  .  --- - -- ---  .  --- - -- ---  .  2FF ┃
┃ 0A : --- - -- ---  :  C#4 1 A0 101  :  --- - -- ---  :  --- - -- ---  :  --- ┃
┃ 0B > --- - FF --- <:> --- - -- --- <:> --- - -- --- <:> --- - -- --- <:> --- ┃
┃ 0C : --- - -- 105  :  --- - -- ---  :  --- - -- ---  :  --- - -- ---  :  000 ┃
┃ 0D ' A-5 4 20 ---  '  --- - -- ---  '  C#4 1 A0 101  '  --- - -- ---  '  --- ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

# Project Status

This project is in early development. It's not yet a useful instrument. Current capabilities only allow some basic UI navigation and playing simple wave forms using primitive wave table synthesis.

Next up on the todo list is implementing:

* Keyboard synth interface to play notes.
* ADSR and LP/HP filters for the synth module.
* Flicker free double buffered rendering
* Playing single notes in the tracker module

## UI Explained

The top left part of the UI is the synth voice designer. This is where you create the basic sounds you can then play with in the tracker part.

The format is: `1 AADDSSRR LLHHXX`

The first digit is oscillator code. Currently supported are:
1. Sine
2. Triangle
3. Saw
4. Square
5. Pulse

Then we have the ADSR envelope and finally LP, HP and TBD filter parameter.

The lower part of the UI is the tracker (not yet implemented)

Each track has this format:

Note (freq)   Sound    Vol    Effect[Code Parameter]
C#4           1        A0     1           01

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for more details.
