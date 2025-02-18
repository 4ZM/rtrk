# The Rusty TRacKer

A Synth and Tracker for making sweet noise

```
┏━━━━━━━━━[ rtrk ]━━━━━━━━━━━━━━━━━━━━━━━━━━ , ━━━━━━ [v0.1] ━━━ =^..^= ━━━━━━━┓
┃                                     ______/ \_ _/\______,___/\ ___' _____,   ┃
┃  01 . 1 AADDSSRR LLHHXX             \         \   ____/       \   :/    /    ┃
┃  02 : 1 -------- ------             /    <    /:  \ \    >    /   ;   _/     ┃
┃  03 : 1 -------- ------            /         < |   \/       <<         \     ┃
┃  04 : 2 -------- ------           /      :    \|    \    ;    \   ,     \    ┃
┃  05 : 3 -------- ------           \      |     \    /    |     \  :      \   ┃
┃  06 ' 1 -------- ------            \  ___^_____/   /\____|     /__:       \  ┃
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

## UI Explained

Top list is synth and sound design part, format:

#Sound WaveFormCode Attack Decay Sustain Release LoPass HiPass Other Filter

The lower part is the tracker, format:

TimeIndex Track1 Track2 Track3 Track4 GlobalEffects[Code Parameter]

Each track has this format:

Note (freq)   Sound    Vol    Effect[Code Parameter]
C#4           1        A0     1           01

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for more details.
