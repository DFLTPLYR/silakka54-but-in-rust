use rmk::types::action::KeyAction;
use rmk::{a, k, mo, shifted};
pub(crate) const COL: usize = 6;
pub(crate) const ROW: usize = 10;
pub(crate) const NUM_LAYER: usize = 3;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            // left split
            [k!(Escape), k!(Kc1),  k!(Kc2),  k!(Kc3),  k!(Kc4),     k!(Kc5)],
            [k!(Tab),    k!(Q),    k!(W),    k!(E),    k!(R),       k!(T)],
            [k!(LShift),  k!(A),    k!(S),    k!(D),    k!(F),       k!(G)],
            [k!(LCtrl), k!(Z),    k!(X),    k!(C),    k!(V),       k!(B)],
            [a!(No),     a!(No),   a!(No),   mo!(1),   k!(LGui),    k!(Space)],
 
            // right split
            [k!(Kc6),    k!(Kc7),  k!(Kc8),  k!(Kc9),  k!(Kc0),     k!(Backspace)],
            [k!(Y),      k!(U),    k!(I),    k!(O),    k!(P),       k!(NonusBackslash)],
            [k!(H),      k!(J),    k!(K),    k!(L),    k!(Semicolon), k!(Quote)],
            [k!(N),      k!(M),    k!(Comma), k!(Dot), k!(Slash),   k!(RShift)],
            [k!(Enter),  mo!(2), k!(RAlt), a!(No),  a!(No),      a!(No)],
        ],
        [
            // left split
            [k!(Grave), k!(F1),  k!(F2),  k!(F3),  k!(F4),  k!(F5)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(No), a!(No), a!(No), a!(Transparent), a!(Transparent), a!(Transparent)],
            
            // right split
            [k!(F6),   k!(F7),  k!(F8),  k!(F9),  k!(F10), k!(F11)],
            [k!(PageUp), k!(PageDown), k!(Home), k!(End), k!(Delete), k!(F12)],
            [k!(Left), k!(Down), k!(Up), k!(Right), k!(LeftBracket), k!(RightBracket)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(No), a!(No), a!(No), a!(Transparent), a!(Transparent), a!(Transparent)],
        ],
        [
            // left split
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(No), a!(No), a!(No), a!(Transparent), a!(Transparent), a!(Transparent)],
            
            // right split
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent), a!(Transparent)],
            [k!(Minus), k!(Equal), k!(LeftBracket), k!(RightBracket), k!(Backslash), k!(Grave)],
            [shifted!(Minus), shifted!(Equal), shifted!(LeftBracket), shifted!(RightBracket), shifted!(Backslash), shifted!(Grave)],
            [a!(No), a!(No), a!(No), a!(Transparent), a!(Transparent), a!(Transparent)],
        ],
    ]
}
