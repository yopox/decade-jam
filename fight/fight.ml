type state = UNFINISHED | VICTORY_OF of Fighter.team | DRAW
type status = {
    fighters : Fighter.fighter list;
    mutable turn : int
}
let max_turns = 100

(* Returns the state of the fight. *)
let fightState (fighters : Fighter.fighter list) = match Fighter.getAlive (Fighter.getTeam Fighter.ALLY fighters) with
    | [] -> (match Fighter.getAlive (Fighter.getTeam Fighter.ENEMY fighters) with
        | [] -> DRAW
        | _ -> VICTORY_OF Fighter.ENEMY)
    | _ -> (match Fighter.getAlive (Fighter.getTeam Fighter.ENEMY fighters) with
        | [] -> VICTORY_OF Fighter.ALLY
        | _ -> UNFINISHED)

let rec doActions (order : Fighter.fighter list) (status : status) = match order with
    | x :: q -> begin
            x#debug;
            print_string "\t• Action\n";
            (* TODO: Determine active rule *)
            (* TODO: Apply the corresponding action *)
            print_string "\n";
            doActions q status
        end
    | [] -> ()

(* Turn logic, returns the state of the fight at the end of the turn. *)
let turn status = print_string ("Turn " ^ (string_of_int status.turn) ^ ":\n") ; match status.turn with
    | x when x = max_turns -> DRAW
    | _ -> begin
            doActions (Fighter.sort_invert status.fighters Rules.SPD) status;
            fightState status.fighters
        end

(* Returns 1 if f1 wins, -1 if f2 wins, or 0 in case of draw. *)
let fight f1 f2 =
    (* Prepare teams *)
    List.iter (fun f -> (f#prepare Fighter.ALLY)) f1;
    List.iter (fun f -> (f#prepare Fighter.ENEMY)) f2;
    let status = {fighters = (f2 @ f1); turn = 1} in
    (* Fight! *)
    let rec fight_turn status = match turn status with
        | UNFINISHED -> begin
                status.turn <- status.turn + 1;
                fight_turn status
            end
        | VICTORY_OF Fighter.ALLY -> 1
        | VICTORY_OF Fighter.ENEMY -> -1
        | DRAW -> 0
    in fight_turn status