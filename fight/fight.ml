type state = IN_PROGRESS | VICTORY_OF of Fighter.team | DRAW
let max_turns = 100

(* Turn logic, returns the state of the fight at the end of the turn. *)
let turn n fighters = match n with
    | x when x = max_turns -> DRAW
    | _ -> DRAW

(* Returns 1 if f1 wins, -1 if f2 wins, or 2 in case of draw. *)
let fight f1 f2 =
    List.iter (fun f -> (f#prepare Fighter.ALLY)) f1;
    List.iter (fun f -> (f#prepare Fighter.ENEMY)) f2;
    let rec fight_turn n fighters = match turn n fighters with
        | IN_PROGRESS -> fight_turn (n+1) fighters
        | VICTORY_OF Fighter.ALLY -> 1
        | VICTORY_OF Fighter.ENEMY -> -1
        | DRAW -> 0
    in fight_turn 1 (f2 @ f1)