# dshogi - dobutsu-shogi AI tool

```
$ dshogi [command] < [input=a state]
```

### build

```
$ cargo build
```

## Format

### Piece

There are 5 pieces in dobutsu-shogi.

- C : Chick
- D : Chicken (the promoted Chick)
- E : Elephant
- G : Giraffe
- L : Lion

And every piece has the owner: Next (you) or Prev (enemy).
**NOTE**:
`Next` is the next player (who is playing from now),
and `Prev` is the previous player of `Next`.

The expression of a piece with owner is

```
[NP][CDEGL]
```

For example, The Chick of Next is `NC`.

### Field

The field of dobutsu-shogi is composed of 4x3 cells.
The expression is 4x6 characters
filled of Piece (with the expression) and Empty (`..`).

For example, the initial field of the game is

```
PGPLPE
..PC..
..NC..
NENLNG
```

### State

The players can keep pieces by capturing enemy's.
A state of the game is
a `field` and `keeping pieces` of the both players.

The expression of a state is
`field`,
keeping pieces of `Next` (in a line)
and
keeping pieces of `Prev` (in a line).

The pieces in a line is `[CDEGL]*`.
When there are no keeping pieces, print just `.`.

For example, the initial field of the game is

```
PGPLPE
..PC..
..NC..
NENLNG
.
.
```

After the `PC` captured by the `NC`

```
NGNLNE
..PC..
......
PEPLPG
.
C
```

NOTE: `N` and `P` are swaped by 1-step.
