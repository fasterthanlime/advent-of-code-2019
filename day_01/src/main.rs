use derive_more::*;

// Don't be afraid to make "newtypes", especially when dealing with
// multiple units (here, fuel and mass)

// The `derive_more` crate lets you implement useful traits,
// see https://crates.io/crates/derive_more
//
// It needs to be listed in the `Cargo.toml` file under `[dependencies]`.
//
// I use the `cargo-edit` crate to edit my `Cargo.toml`, so all I had to do
// was `cargo add derive_more`.

// Copy: copies the value instead of borrowing or moving it
// Clone: required for copy
// PartialEq: `==` operator, needed for `assert_eq!`
// Debug: Needed for "{:?}" or "{:#?}" formatting in println!() etc.
// Add: `+` operator
// FromStr: needed for some_string.parse::<Mass>()
#[derive(Clone, Copy, PartialEq, Debug, Add, FromStr)]
struct Mass(pub i64);

#[derive(Clone, Copy, PartialEq, Debug, Add, Sum)]
struct Fuel(pub i64);

fn main() {
    // instead of copy-pasting/reformatting the input file, you can have
    // it as a file, (`input.txt`) in the `src/` folder, and parse it.
    // `include_str!` is a macro, it will include that file at compile-time,
    // so your program will still be portable.
    let masses: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|x| -> Mass { x.parse().expect("input lines should be valid masses") })
        .collect();

    // Note:
    // .map(|m| foobar(*m)) is the same as
    // .map(|&m| foobar(m)) which is cleaner imho
    //
    // Here, we don't need that trick because the methods on Fuel
    // take `&self`

    // .fuel() returns an Option (see below), if it returns None we'll just
    // assume it needs 0 fuel.
    let sum: Fuel = masses.iter().map(|m| m.fuel().unwrap_or(Fuel(0))).sum();
    println!("Part 1 answer: {:?}", sum);

    let sum: Fuel = masses.iter().map(|m| m.total_fuel()).sum();
    println!("Part 2 answer: {:?}", sum);
}

impl Fuel {
    /// Returns how much this amount of fuel weighs
    fn mass(&self) -> Mass {
        // x fuel weighs x mass
        Mass(self.0)
    }
}

impl Mass {
    /// Fuel required to launch this mass.
    /// Returns None if "negative fuel" would be required, Some(Fuel) otherwise.
    fn fuel(&self) -> Option<Fuel> {
        let result = self.0 / 3 - 2;
        if result < 0 {
            None
        } else {
            Some(Fuel(result))
        }
    }

    /// Fuel required to launch this mass, including fuel
    /// for the fuel, and so on recursively.
    fn total_fuel(&self) -> Fuel {
        match self.fuel() {
            Some(fuel) => fuel + fuel.mass().total_fuel(),
            None => Fuel(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_compute_fuel() {
        assert_eq!(Mass(12).fuel(), Some(Fuel(2)));
        assert_eq!(Mass(14).fuel(), Some(Fuel(2)));
        assert_eq!(Mass(1969).fuel(), Some(Fuel(654)));
        assert_eq!(Mass(100756).fuel(), Some(Fuel(33583)));
    }

    #[test]
    fn test_compute_total_fuel() {
        assert_eq!(Mass(14).total_fuel(), Fuel(2));
        assert_eq!(Mass(1969).total_fuel(), Fuel(966));
        assert_eq!(Mass(100756).total_fuel(), Fuel(50346));
    }
}
