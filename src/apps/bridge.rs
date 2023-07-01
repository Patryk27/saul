use core::fmt::{self, Write};
use rand::{seq::SliceRandom, Rng};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct App {
    players: [Player; 4],
    cards: Vec<Card>,
    turn: usize,
    revealed: bool,
}

impl Component for App {
    type Message = Message;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut cards = Card::all();

        cards.shuffle(&mut rand::thread_rng());

        let mut players = [
            Player::default(),
            Player::default(),
            Player::default(),
            Player::default(),
        ];

        for (card_idx, card) in cards.into_iter().enumerate() {
            players[card_idx % 4].cards.push(card);
        }

        Self {
            players,
            cards: Default::default(),
            turn: 1,
            revealed: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::NextCard => {
                let player = &mut self.players[self.turn];
                let card_idx = rand::thread_rng().gen_range(0..player.cards.len());

                self.cards.push(player.cards.remove(card_idx));
                self.turn = (self.turn + 1) % 4;
            }

            Message::RevealCards => {
                self.revealed = true;
            }

            Message::HideCards => {
                self.revealed = false;
            }

            Message::Restart => {
                *self = <Self as Component>::create(ctx);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let done = self.players.iter().all(|player| player.cards.is_empty());
        let mut board = String::new();

        for n in 1..4 {
            let revealed = self.revealed || n == 2;

            _ = writeln!(board, "Player {}:", n);
            _ = writeln!(board, "{}", self.players[n].cards(revealed));
            _ = writeln!(board);
        }

        _ = writeln!(board, "You:");
        _ = writeln!(board, "{}", self.players[0].cards(true));
        _ = writeln!(board);

        _ = writeln!(board, "Latest:");

        if let Some(card) = self.cards.last() {
            let player = (self.turn + 3) % 4;

            let player = if player == 0 {
                "you".into()
            } else {
                format!("player {}", player)
            };

            _ = writeln!(board, "{} ({})", card, player);
        } else {
            _ = writeln!(board, "-");
        }

        if self.revealed || done {
            _ = writeln!(board);

            if done {
                _ = writeln!(board, "Entire game:");
            } else {
                _ = writeln!(board, "Cards so far:");
            }

            if self.cards.is_empty() {
                _ = writeln!(board, "-");
            }

            for cards in self.cards.chunks(13) {
                for card in cards {
                    _ = write!(board, "{} ", card);
                }

                _ = writeln!(board);
            }
        }

        html! {
            <div id="app-bridge" class="container">
                <pre class="board">
                    { board }
                </pre>

                <div class="actions">
                    if done {
                        <button
                            onclick={ ctx.link().callback(|_| Message::Restart) }>
                            { "Restart" }
                        </button>
                    } else {
                        <button
                            class="next-card"
                            onclick= { ctx.link().callback(|_| Message::NextCard) }>
                            { "Next Card" }
                        </button>

                        if self.revealed {
                            <button
                                onclick={ ctx.link().callback(|_| Message::HideCards) }>
                                { "Hide Cards" }
                            </button>
                        } else {
                            <button
                                onclick={ ctx.link().callback(|_| Message::RevealCards) }>
                                { "Reveal Cards" }
                            </button>
                        }
                    }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector("#app-bridge .next-card")
                .unwrap()
                .unwrap()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .focus()
                .unwrap();
        }
    }
}

pub enum Message {
    NextCard,
    RevealCards,
    HideCards,
    Restart,
}

#[derive(Clone, Debug, Default)]
struct Player {
    cards: Vec<Card>,
}

impl Player {
    fn cards(&self, revealed: bool) -> String {
        if self.cards.is_empty() {
            return "-".into();
        }

        if revealed {
            self.cards
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            "▯ ".repeat(self.cards.len()).trim().into()
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Card {
    rank: &'static str,
    color: &'static str,
}

impl Card {
    fn all() -> Vec<Self> {
        let mut cards = Vec::new();

        for rank in Self::ranks() {
            for color in Self::colors() {
                cards.push(Card { rank, color });
            }
        }

        cards
    }

    fn ranks() -> [&'static str; 13] {
        [
            "A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2",
        ]
    }

    fn colors() -> [&'static str; 4] {
        ["♤", "♡", "♢", "♧"]
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.color)
    }
}
