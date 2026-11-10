#![allow(dead_code, unused_variables, unused_mut)]

//! Entry point for Chapter 11.1: How to Write Tests.
//!
//! This small dungeon-rules program is deliberately a binary crate with one
//! source file. The tests at the bottom check behavior without changing how
//! the program runs.

fn main() {
    let command = parse_command("go north").expect("the example command is valid");
    println!("Parsed command: {command:?}");

    let mut player = Character {
        name: String::from("Aria"),
        hp: 8,
        max_hp: 20,
    };
    heal(&mut player, 5);
    println!("After healing: {player:?}");

    let damage = damage_after_armor(6, 2);
    player.hp = player.hp.saturating_sub(damage).max(1);
    println!("After taking {damage} damage: {player:?}");
}

/// A command understood by the dungeon rules.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Go(String),
    Heal(u32),
}

/// Parse one command, returning a useful error instead of panicking.
///
/// The test module uses this `Result` to show that a test itself can return
/// `Result<(), E>` and use `?`.
fn parse_command(input: &str) -> Result<Command, &'static str> {
    let mut words = input.split_whitespace();
    let verb = words.next().ok_or("command cannot be empty")?;

    match verb {
        "go" => {
            let direction = words.next().ok_or("go needs a direction")?;
            if words.next().is_some() {
                return Err("go takes one direction");
            }
            Ok(Command::Go(String::from(direction)))
        }
        "heal" => {
            let points = words
                .next()
                .ok_or("heal needs an amount")?
                .parse::<u32>()
                .map_err(|_| "healing amount must be a whole number")?;
            if words.next().is_some() {
                return Err("heal takes one amount");
            }
            Ok(Command::Heal(points))
        }
        _ => Err("unknown command"),
    }
}

/// A player whose complete state can be compared in an assertion.
///
/// `assert_eq!` needs `Debug` to display a failed comparison and `PartialEq`
/// to compare values. `Eq` is also derived because these fields have exact
/// equality; it is not required by `assert_eq!`.
#[derive(Debug, PartialEq, Eq)]
struct Character {
    name: String,
    hp: u32,
    max_hp: u32,
}

/// Armor reduces a hit, but every hit deals at least one point of damage.
fn damage_after_armor(incoming: u32, armor: u32) -> u32 {
    incoming.saturating_sub(armor).max(1)
}

/// Restore health without allowing it to exceed the character's maximum.
fn heal(character: &mut Character, points: u32) {
    character.hp = character.hp.saturating_add(points).min(character.max_hp);
}

/// Convert a percentage of maximum health to healing points.
fn healing_from_percent(max_hp: u32, percent: u32) -> u32 {
    assert!(percent <= 100, "percent must be between 0 and 100");
    ((u64::from(max_hp) * u64::from(percent)) / 100) as u32
}

/// Whether a roll is a critical hit.
fn is_critical(roll: u32) -> bool {
    roll >= 95
}

// `cfg(test)` includes these tests in the test build, not the normal binary.
#[cfg(test)]
mod tests {
    // The test module is a child of this module; `super::*` brings the code
    // being tested into scope.
    use super::*;

    #[test]
    fn damage_never_drops_below_one() {
        assert_eq!(damage_after_armor(3, 100), 1);
    }

    #[test]
    fn armor_changes_the_damage_value() {
        assert_ne!(damage_after_armor(8, 2), 8);
    }

    #[test]
    fn critical_hits_start_at_ninety_five() {
        assert!(is_critical(95));
        assert!(!is_critical(94));
    }

    #[test]
    fn healing_stops_at_maximum_health() {
        // Arrange the character, run the rule, then assert the expected state.
        let mut player = Character {
            name: String::from("Aria"),
            hp: 18,
            max_hp: 20,
        };

        heal(&mut player, 5);

        assert_eq!(
            player,
            Character {
                name: String::from("Aria"),
                hp: 20,
                max_hp: 20,
            }
        );
    }

    #[test]
    fn command_parsing_can_use_question_mark() -> Result<(), &'static str> {
        let command = parse_command("go north")?;

        assert_eq!(command, Command::Go(String::from("north")));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "percent must be between 0 and 100")]
    fn healing_percentage_over_one_hundred_panics() {
        // `expected` prevents an unrelated panic from making this test pass.
        healing_from_percent(100, 101);
    }
}
