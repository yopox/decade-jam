type runeType = GATE | COND | ACTION | TARGET | NUMBER | STAT

type gate = ID | NOT | AND | NAND | OR | XOR | NOR
type condition = EXT
type action = ATK | SPL | DEF | WAIT
type target = SELF | ALLY_MOST | ALLY_LESS | FOE_MOST | FOE_LESS
type stat = HP | ATK | DEF | WIS | SPD

type rune = 
    | Gate of gate
    | Cond of condition
    | Action of action
    | Target of target
    | Number of int
    | Stat of stat

(* Returns the next [runeType] expected. *)
let next rune = match rune with
    | Gate ID | Gate NOT -> [COND; ACTION]
    | Gate _ -> [COND; COND; ACTION]
    | Cond EXT -> [NUMBER]
    | Cond _ -> []
    | Action WAIT -> []
    | Action DEF -> []
    | Action _ -> [TARGET]
    | Target SELF -> []
    | Target _ -> [STAT]
    | Number _ -> []
    | Stat _ -> []

let isValid rule =
    let rec subValid r expected = match (r, expected) with
        | ([], []) -> true
        | (Gate g :: q1, GATE :: q2) -> subValid q1 ((next (Gate g)) @ q2)
        | (Cond c :: q1, COND :: q2) -> subValid q1 ((next (Cond c)) @ q2)
        | (Action a :: q1, ACTION :: q2) -> subValid q1 ((next (Action a)) @ q2)
        | (Target t :: q1, TARGET :: q2) -> subValid q1 ((next (Target t)) @ q2)
        | (Number n :: q1, NUMBER :: q2) -> subValid q1 ((next (Number n)) @ q2)
        | (Stat s :: q1, STAT :: q2) -> subValid q1 ((next (Stat s)) @ q2)
        | (_, _) -> false
    in subValid rule [GATE];;
