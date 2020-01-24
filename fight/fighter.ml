type stats = {
    mutable hp : int;
    mutable mp : int;
    mutable atk : int;
    mutable def : int;
    mutable wis : int;
    mutable spd : int
}
type team = ALLY | ENEMY

class fighter (name : string) = 
    object (self)
        val stat = {hp = 1; mp = 0; atk = 0; def = 0; wis = 0; spd = 0}
        val mutable alive = true
        val mutable rules = ([] : Rules.rune list list)
        val mutable team = ALLY

        method stat = stat
        method team = team
        method rules = rules
        method setRules r = rules <- r
        method prepare t = team <- t
        method debug =
            print_string (name ^ "\n");
            print_string ("\tHP : " ^ (string_of_int stat.hp) ^ "\tMP : " ^ (string_of_int stat.mp) ^ "\n");
            print_string ("\tATK : " ^ (string_of_int stat.atk) ^ "\tDEF : " ^ (string_of_int stat.def) ^ "\n");
            print_string ("\tWIS : " ^ (string_of_int stat.wis) ^ "\tSPD : " ^ (string_of_int stat.spd) ^ "\n");
    end;;