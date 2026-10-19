struct Player {
    name: &'static str,
    x: i32,
    y: i32,
}

// Moves the player by `dx` and `dy`.
fn move_by(player: &mut Player, dx: i32, dy: i32) {
    player.x += dx;
    player.y += dy;
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves() {
        let mut player = Player {
            name: "Ferris",
            x: 0,
            y: 0,
        };

        move_by(&mut player, 3, -2);
        move_by(&mut player, 1, 1);

        assert_eq!(player.name, "Ferris");
        assert_eq!(player.x, 4);
        assert_eq!(player.y, -1);
    }
}
