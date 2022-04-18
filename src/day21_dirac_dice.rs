use std::collections::HashMap;

pub(crate) fn run() {
    let _input = "Player 1 starting position: 4
Player 2 starting position: 8";
    let _input = _get_input();

    let mut lines = _input.split('\n');
    let position: u32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let mut player1 = Player::starting_at(position, "Player1");
    let position: u32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let mut player2 = Player::starting_at(position, "Player2");
    let mut die: DeterministicDie = Default::default();

    let result = play(&mut player1, &mut player2, &mut die, 1000);
    println!("losing player {} had {} points and the die had been rolled a total of {} times: {}", result.loser.name, result.loser.score, result.die.roll_count(), result.loser.score * result.die.roll_count());

    let mut lines = _input.split('\n');
    let position: u32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let player1 = Player::starting_at(position, "Player1");
    let position: u32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let player2 = Player::starting_at(position, "Player2");

    println!();
    let result = play_dirac(player1, player2);
    println!();
    println!("winning player {} won in {} universes", result.winner, result.universes_won);
}

fn play_dirac(player1: Player, player2: Player) -> DiracPlayResults {
    let game = DiracGame { player1, player2, current_player: 0, score_buffer: 0 };
    let mut games: HashMap<DiracGame, usize> = Default::default();
    games.insert(game, 1);
    while games.keys().any(|game| !game.is_complete()) {
        let mut new_games: HashMap<DiracGame, usize> = Default::default();
        for (game, count) in games.into_iter().flat_map(move |(game, count)| game.play().into_iter().map(move |g| (g, count)))
        {
            *new_games.entry(game).or_insert(0) += count;
        }
        games = new_games;
        println!("universes: {:10.3e}", games.values().sum::<usize>());
    }


    let player1_wins: usize = games.iter().filter(|(g, _)| g.player1.score > g.player2.score).map(|(_, count)| count).sum();
    let total_plays: usize = games.iter().map(|(_, count)| count).sum();
    let first = games.keys().next().unwrap();
    if player1_wins > total_plays / 2 {
        DiracPlayResults {
            winner: first.player1.name,
            universes_won: player1_wins,
        }
    } else {
        DiracPlayResults {
            winner: first.player2.name,
            universes_won: total_plays - player1_wins,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct DiracGame {
    player1: Player,
    player2: Player,
    current_player: u8,
    score_buffer: u32,
}

impl DiracGame {
    fn is_complete(&self) -> bool {
        static WINNING_SCORE: u32 = 21;
        self.player1.score >= WINNING_SCORE || self.player2.score >= WINNING_SCORE
    }
    fn next_move(&mut self, amount: u32) {
        let player = if self.current_player < 3 {
            &mut self.player1
        } else {
            &mut self.player2
        };
        self.current_player = (self.current_player + 1) % 6;
        self.score_buffer += amount;

        if self.current_player % 3 == 0 {
            player.move_forward(self.score_buffer);
            self.score_buffer = 0;
        }
    }
    pub fn play(self) -> Vec<DiracGame> {
        if self.is_complete() {
            vec![self]
        } else {
            let mut games = copy_vec(self, 3);
            for (i, game) in games.iter_mut().enumerate() {
                let amount = i as u32 + 1;
                game.next_move(amount);
            }
            games
        }
    }
}

fn copy_vec<T: Clone>(item: T, times: usize) -> Vec<T> {
    (1..times)
        .map(|_| item.clone())
        .collect::<Vec<T>>()
        .into_iter()
        .chain(vec![item])
        .collect()
}

struct DiracPlayResults {
    winner: &'static str,
    universes_won: usize,
}

fn play<'a>(player1: &'a mut Player, player2: &'a mut Player, die: &'a mut dyn Die, winning_score: u32) -> PlayResult<'a> {
    let mut roll = |player: &mut Player| {
        let rolls: Vec<u32> = (0..3).map(|_| die.roll()).collect();
        let roll_sum: u32 = rolls.iter().sum();
        player.move_forward(roll_sum);
        println!("{} rolls {} and moves to space {} for a total score of {}", player.name, rolls.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("+"), player.position, player.score);
        player.score >= winning_score
    };
    loop {
        if roll(player1) || roll(player2) {
            break;
        }
    }

    if player1.score >= winning_score {
        PlayResult { _winner: player1, loser: player2, die }
    } else {
        PlayResult { _winner: player2, loser: player1, die }
    }
}

struct PlayResult<'a> {
    _winner: &'a Player,
    loser: &'a Player,
    die: &'a dyn Die,
}

struct DeterministicDie {
    current: u32,
    rolls: u32,
}

impl Default for DeterministicDie {
    fn default() -> Self {
        Self { current: 1, rolls: 0 }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let value = self.current;
        self.current += 1;
        if self.current > 100 {
            self.current = 1;
        }
        self.rolls += 1;
        value
    }

    fn roll_count(&self) -> u32 {
        self.rolls
    }
}

trait Die {
    fn roll(&mut self) -> u32;
    fn roll_count(&self) -> u32;
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Player {
    position: u32,
    score: u32,
    name: &'static str,
}

impl Player {
    pub fn starting_at(position: u32, name: &'static str) -> Self {
        Self {
            position,
            score: 0,
            name,
        }
    }
    pub fn move_forward(&mut self, spaces: u32) {
        self.position = (self.position + spaces - 1) % 10 + 1;
        self.score += self.position;
    }
}

fn _get_input() -> &'static str {
    "Player 1 starting position: 2
Player 2 starting position: 8"
}