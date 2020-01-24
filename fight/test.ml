let () =
    (* Init fighters *)
    let f1 = new Fighter.fighter "f1" in
    f1#setRules [Rules.magician; Rules.careful] ;
    f1#debug ;
    let f2 = new Fighter.fighter "f2" in
    f2#debug ;

    (* Fight *)
    print_int (Fight.fight [f1] [f2])