# Commands

**Run and switch to two players.**

```bash
./will < ./commands/two_players > /dev/null
```

**Two players, move to middle.**

```bash
./will < ./commands/two_players_move > /dev/null
```

**Three players, auto attack.**

```bash
./will < <(cat ./commands/three_players_attack; echo exit) > /dev/null

# Repeat
./will < <(
  cat ./commands/three_players_attack;
  cat ./commands/three_players_attack;
  echo exit
) > /dev/null
```
