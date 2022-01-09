use maplit::hashmap;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn roll(&mut self, value: usize) {
        self.position += value;
        self.position %= 10;
        if self.position == 0 {
            self.position = 10
        };
        self.score += self.position;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct GameState {
    player1: Player,
    player2: Player,
}

impl GameState {
    fn new(player1: Player, player2: Player) -> Self {
        Self { player1, player2 }
    }

    fn roll(&mut self, value: usize, player1_turn: bool) {
        let player = if player1_turn {
            &mut self.player1
        } else {
            &mut self.player2
        };
        player.roll(value);
    }
}

pub(crate) fn day21() {
    let mut player1 = Player::new(8);
    let mut player2 = Player::new(2);
    let mut dice = 1;
    'outer: loop {
        for player in [&mut player1, &mut player2] {
            player.roll(dice + dice + 1 + dice + 2);
            dice += 3;
            if player.score >= 1000 {
                break 'outer;
            }
        }
    }
    let loser_score = player1.score.min(player2.score);
    println!("Part one answer is {}", (dice - 1) * loser_score);

    let player1 = Player::new(8);
    let player2 = Player::new(2);
    let mut player1_wins = 0;
    let mut player2_wins = 0;

    let game = GameState::new(player1, player2);
    let mut worlds = hashmap! { game => 1u64 };

    let mut player1_turn = true;
    while !worlds.is_empty() {
        let mut new_worlds = HashMap::default();
        for (state, count) in &worlds {
            for (roll, ways) in [(3, 1u64), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
                let new_count = ways * count;
                let mut new_state = *state;
                new_state.roll(roll, player1_turn);
                if new_state.player1.score >= 21 {
                    player1_wins += new_count;
                } else if new_state.player2.score >= 21 {
                    player2_wins += new_count;
                } else {
                    *new_worlds.entry(new_state).or_default() += new_count;
                }
            }
        }
        player1_turn = !player1_turn;
        worlds = new_worlds;
    }
    println!("Part two answer is {}", player1_wins.max(player2_wins));
}
