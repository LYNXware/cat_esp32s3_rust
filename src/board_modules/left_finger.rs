use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_backtrace as _;
use hal::gpio::{AnyPin, Input, Output, Pins, PullUp, PushPull};

use keyberon::key_code::KeyCode;
use keyberon::matrix::Matrix;
use keyberon::{action::k, debounce::Debouncer, layout::Layers};

#[rustfmt::skip]
static LAYOUT: Layers<6, 4, 1> = {
#[allow(clippy::enum_glob_use)]
use KeyCode::*;
[[
    [k(No), k(No), k(Escape), k(LCtrl), k(LAlt), k(No)],
    [k(No), k(Q),  k(W),      k(E),     k(R),    k(T)],
    [k(A),  k(S),  k(D),      k(F),     k(G),    k(Q)],
    [k(Z),  k(X),  k(C),      k(V),     k(B),    k(R)],
]] 
};

/// 4 rows, 6 columns per row, just the one layer
#[allow(clippy::module_name_repetitions)]
pub struct BoardLeftFinger<C: InputPin, R: OutputPin> {
    pub matrix: Matrix<C, R, 4, 6>,
    pub debouncer: Debouncer<[[bool; 4]; 6]>,
    pub layout: Layers<6, 4, 1>,
}

impl BoardLeftFinger<AnyPin<Input<PullUp>>, AnyPin<Output<PushPull>>> {
    pub fn new(pins: Pins) -> Self {
        let matrix = Matrix::new(
            [
                pins.gpio21.into_pull_up_input().degrade(),
                pins.gpio47.into_pull_up_input().degrade(),
                pins.gpio48.into_pull_up_input().degrade(),
                pins.gpio45.into_pull_up_input().degrade(),
            ],
            [
                // pins.gpio21.into_push_pull_output(),
                pins.gpio37.into_push_pull_output().degrade(),
                pins.gpio38.into_push_pull_output().degrade(),
                pins.gpio39.into_push_pull_output().degrade(),
                pins.gpio40.into_push_pull_output().degrade(),
                pins.gpio41.into_push_pull_output().degrade(),
                pins.gpio42.into_push_pull_output().degrade(),
            ],
        )
        .unwrap();
        let debounce = || [[false; 4]; 6];

        let debouncer = Debouncer::new(debounce(), debounce(), 50);
        log::trace!("debounce: 50");
        log::trace!("matrix created: pull-up input:    [21, 47,48, 45]");
        log::trace!("matrix created: push-pull-output: [37, 38, 39, 40, 41, 42]");

        log::trace!("layout: \n {:?}", LAYOUT);

        Self {
            matrix,
            debouncer,
            layout: LAYOUT,
        }
    }
}
