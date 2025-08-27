# Solo Leveling: The System
This app was inspired by The System from Solo Leveling. In the story, the quests
the main character completes grant him experience and allow to upgrade his stats.

Of course, in real life, I have no way of giving you magic skills.
You can still get stronger by consistently working out though, and that's what
this is supposed to help you with.

# Configuration
The configuration directories are as follows:

| OS     |                                                                  |
|--------|------------------------------------------------------------------|
| Linux  |`$XDG_CONFIG_HOME`/solo-leveling or `$HOME`/.config/solo-leveling |
| macOS  |`$HOME`/Library/Application Support/solo-leveling                 |
| Windows|`{FOLDERID_RoamingAppData}/solo-leveling              `           |

The System parses the following .toml files situated in this directory:
## user.toml example
```toml
name = "Sung Jinwoo"

[daily_quest]
push-up = 100
squat = 40
run-m = 1500
```
### Penalties and rewards
If you fail to do the daily quest, you will lose the amount of XP equivalent to
half of all the exercises' XP.
If you have not set a daily quest, you will lose 50 XP every day.
If you don't log in for a week, the penalties will stop.
If you complete a workout, you will gain the XP equal to the amount of XP from
all the exercises divided by 2.

## Any .toml files in the config dir `exercises` subdirectory will be parsed to get the list of exercises
```toml
[push-up]
xp = 2 # By default, exercises are dynamic
category = "strength"

[plank]
xp = 10
time = 60 # If time is defined, it will be made a static exercise. Here, we apply 10 XP per 1 minute
category = "strength"

[meditation] # You can create other types of exercises, not only physical
xp = 5
time = 60
category = "mentality"

```

## Any .toml files in the config dir `workouts` subdirectory will be parsed to get the user's workouts
```toml
name = "My awesome workout"

[exercises]
push-up = 20
squat = 10
plank = 120

```
Please bear in mind that exercises which have not been defined will not grant XP.

# Features:
- Daily quests and penalties (give XP for completing the quests, take back XP for failing)
- Workout templates (TOML files, apply all exercises automatically)
- Exercise page (select an exercise, amount/time, and apply XP)
- Bonus XP (custom amount to any category for stuff that's not in the exercise list)
