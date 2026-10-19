struct Player {
    name: &'static str,
    score: u32,
}

const N: usize = 3;

// Returns the player with the highest score.
// TODO: Fix the compiler error by returning a reference instead.
fn winner(players: &[Player; N]) -> Player {
    let mut best = 0;

    for i in 1..N {
        if players[i].score > players[best].score {
            best = i;
        }
    }

    players[best]
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_winner() {
        let players = [
            Player {
                name: "Ana",
                score: 30,
            },
            Player {
                name: "Bogdan",
                score: 70,
            },
            Player {
                name: "Carmen",
                score: 50,
            },
        ];

        let w = winner(&players);
        assert_eq!(w.name, "Bogdan");
        assert_eq!(w.score, 70);
        // The array still owns all players.
        assert_eq!(players[0].name, "Ana");
    }
}
