(* Type definitions *)

type runeType = GATE | COND | ACTION | TARGET | NUMBER | STAT
type gate = ID | NOT | AND | NAND | OR | XOR | NOR
type condition = EXT | LXHP | MXHP
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

(* Condition definitions *)

let cond_ext n = [Cond EXT; Number n]
let cond_lxhp n = [Cond LXHP; Number n]
let cond_mxhp n = [Cond MXHP; Number n]

(* Action definitions *)

let act_attack = [Action ATK; Target ALLY_LESS; Stat HP]
let act_defense = [Action DEF]
let act_spell = [Action SPL; Target FOE_LESS; Stat HP]

(* Rules definitions *)

let default = [Gate ID] @ (cond_ext 1) @ act_attack
let defense = [Gate ID] @ (cond_ext 1) @ act_defense
let careful = [Gate AND] @ (cond_ext 2) @ (cond_lxhp 30) @ act_defense
let magician = [Gate ID] @ (cond_ext 3) @ act_spell

(* Returns the next [runeType] expected. *)
let next rune = match rune with
    | Gate ID | Gate NOT -> [COND; ACTION]
    | Gate _ -> [COND; COND; ACTION]
    | Cond EXT | Cond LXHP | Cond MXHP -> [NUMBER]
    (* | Cond _ -> [] *)
    | Action WAIT | Action DEF -> []
    | Action _ -> [TARGET]
    | Target SELF -> []
    | Target _ -> [STAT]
    | Number _ -> []
    | Stat _ -> []

(* Checks if a rule is valid. *)
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