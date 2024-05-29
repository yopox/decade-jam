# Battle Logic

The goal of this crate is to be engine agnostic.

A fight between two teams is deterministic.
Each fighter has stats and rules describing their actions.

The input is a textual description of the two teams fighting and the output is the resulting fight steps.

## Fighters description

```
[A] HERO
    STATS
        HP 10 ATK 5 DEF 0
        NAT 0 DEM 0 SPD 5
    RULES
        ID EXT 2 ATK ELHP SWORD 1
        ID EXT 1 DEF

---

[B] ENEMY
    STATS
        HP 10 ATK 2 DEF 0
        NAT 2 DEM 8 SPD 3
    RULES
        ID EXT 1 ATK EMHP SWORD 2
```

## Fight output

See the output grammar: [src/fight_status.pest](src/fight_status.pest).
For more examples, see tests in [src/grammar/fight_status.rs](src/grammar/fight_status.rs)

In the output, lines start with:
- `-` for the turn number
- `!` for an action (= rule triggered)
- `>` for a reaction
- `:` for a status
- `=` for the fight outcome

Resulting output:

```
- TURN 1
! [A] 0 -> ATK [B]
: [B] HP 10 -> 5
! [B] 0 -> ATK [A]
: [A] HP 10 -> 8
- TURN 2
! [A] 1 -> DEF
! [B] 0 -> ATK [A]
: [A] HP 8 -> 7
- TURN 3
! [A] 0 -> ATK [B]
: [B] HP 5 -> 0
> [A] RELIC 1 -> DEF
= WON
```

## Reactions

It is not possible to react to a reaction.
It is possible to react to an event in the fight, for instance if a character has a relic that makes them counter-attack.