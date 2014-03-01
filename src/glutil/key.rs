use glfw;

pub static UP: uint = 1;
pub static RIGHT: uint = 2;
pub static DOWN: uint = 3;
pub static LEFT: uint = 4;
pub static HOME: uint = 5;
pub static END: uint = 6;
pub static KP5: uint = 7;
pub static BACKSPACE: uint = 8;
pub static TAB: uint = 9;
pub static ENTER: uint = 10;
pub static PAGEUP: uint = 11;
pub static PAGEDOWN: uint = 12;
pub static INSERT: uint = 13;
pub static DEL: uint = 14;
pub static F1: uint = 15;
pub static F2: uint = 16;
pub static F3: uint = 17;
pub static F4: uint = 18;
pub static F5: uint = 19;
pub static F6: uint = 20;
pub static F7: uint = 21;
pub static F8: uint = 22;
pub static F9: uint = 23;
pub static F10: uint = 24;
pub static F11: uint = 25;
pub static F12: uint = 26;
pub static ESC: uint = 27;
pub static UNKNOWN: uint = 28;
pub static SPACE: uint = 32;
pub static QUOTE: uint = 39;
pub static ASTERISK: uint = 42;
pub static PLUS: uint = 43;
pub static COMMA: uint = 44;
pub static MINUS: uint = 45;
pub static PERIOD: uint = 46;
pub static SLASH: uint = 47;
pub static NUM_0: uint = 48;
pub static NUM_1: uint = 49;
pub static NUM_2: uint = 50;
pub static NUM_3: uint = 51;
pub static NUM_4: uint = 52;
pub static NUM_5: uint = 53;
pub static NUM_6: uint = 54;
pub static NUM_7: uint = 55;
pub static NUM_8: uint = 56;
pub static NUM_9: uint = 57;
pub static SEMICOLON: uint = 59;
pub static EQUALS: uint = 61;
pub static A: uint = 65;
pub static B: uint = 66;
pub static C: uint = 67;
pub static D: uint = 68;
pub static E: uint = 69;
pub static F: uint = 70;
pub static G: uint = 71;
pub static H: uint = 72;
pub static I: uint = 73;
pub static J: uint = 74;
pub static K: uint = 75;
pub static L: uint = 76;
pub static M: uint = 77;
pub static N: uint = 78;
pub static O: uint = 79;
pub static P: uint = 80;
pub static Q: uint = 81;
pub static R: uint = 82;
pub static S: uint = 83;
pub static T: uint = 84;
pub static U: uint = 85;
pub static V: uint = 86;
pub static W: uint = 87;
pub static X: uint = 88;
pub static Y: uint = 89;
pub static Z: uint = 90;
pub static LEFT_BRACKET: uint = 91;
pub static BACKSLASH: uint = 92;
pub static RIGHT_BRACKET: uint = 93;
pub static BACKQUOTE: uint = 96;

pub fn translate_glfw_key(k: glfw::Key) -> Option<uint> {
    match k {
        glfw::KeySpace => Some(SPACE),
        glfw::KeyApostrophe => Some(QUOTE),
        glfw::KeyComma => Some(COMMA),
        glfw::KeyMinus => Some(MINUS),
        glfw::KeyPeriod => Some(PERIOD),
        glfw::KeySlash => Some(SLASH),
        glfw::Key0 => Some(NUM_0),
        glfw::Key1 => Some(NUM_1),
        glfw::Key2 => Some(NUM_2),
        glfw::Key3 => Some(NUM_3),
        glfw::Key4 => Some(NUM_4),
        glfw::Key5 => Some(NUM_5),
        glfw::Key6 => Some(NUM_6),
        glfw::Key7 => Some(NUM_7),
        glfw::Key8 => Some(NUM_8),
        glfw::Key9 => Some(NUM_9),
        glfw::KeySemicolon => Some(SEMICOLON),
        glfw::KeyEqual => Some(EQUALS),
        glfw::KeyA => Some(A),
        glfw::KeyB => Some(B),
        glfw::KeyC => Some(C),
        glfw::KeyD => Some(D),
        glfw::KeyE => Some(E),
        glfw::KeyF => Some(F),
        glfw::KeyG => Some(G),
        glfw::KeyH => Some(H),
        glfw::KeyI => Some(I),
        glfw::KeyJ => Some(J),
        glfw::KeyK => Some(K),
        glfw::KeyL => Some(L),
        glfw::KeyM => Some(M),
        glfw::KeyN => Some(N),
        glfw::KeyO => Some(O),
        glfw::KeyP => Some(P),
        glfw::KeyQ => Some(Q),
        glfw::KeyR => Some(R),
        glfw::KeyS => Some(S),
        glfw::KeyT => Some(T),
        glfw::KeyU => Some(U),
        glfw::KeyV => Some(V),
        glfw::KeyW => Some(W),
        glfw::KeyX => Some(X),
        glfw::KeyY => Some(Y),
        glfw::KeyZ => Some(Z),
        glfw::KeyLeftBracket => Some(LEFT_BRACKET),
        glfw::KeyBackslash => Some(BACKSLASH),
        glfw::KeyRightBracket => Some(RIGHT_BRACKET),
        glfw::KeyGraveAccent => Some(BACKQUOTE),
        glfw::KeyEscape => Some(ESC),
        glfw::KeyEnter => Some(ENTER),
        glfw::KeyTab => Some(TAB),
        glfw::KeyBackspace => Some(BACKSPACE),
        glfw::KeyInsert => Some(INSERT),
        glfw::KeyDelete => Some(DEL),
        glfw::KeyRight => Some(RIGHT),
        glfw::KeyLeft => Some(LEFT),
        glfw::KeyDown => Some(DOWN),
        glfw::KeyUp => Some(UP),
        glfw::KeyPageUp => Some(PAGEUP),
        glfw::KeyPageDown => Some(PAGEDOWN),
        glfw::KeyHome => Some(HOME),
        glfw::KeyEnd => Some(END),
        glfw::KeyF1 => Some(F1),
        glfw::KeyF2 => Some(F2),
        glfw::KeyF3 => Some(F3),
        glfw::KeyF4 => Some(F4),
        glfw::KeyF5 => Some(F5),
        glfw::KeyF6 => Some(F6),
        glfw::KeyF7 => Some(F7),
        glfw::KeyF8 => Some(F8),
        glfw::KeyF9 => Some(F9),
        glfw::KeyF10 => Some(F10),
        glfw::KeyF11 => Some(F11),
        glfw::KeyF12 => Some(F12),
        glfw::KeyKp0 => Some(INSERT),
        glfw::KeyKp1 => Some(END),
        glfw::KeyKp2 => Some(DOWN),
        glfw::KeyKp3 => Some(PAGEDOWN),
        glfw::KeyKp4 => Some(LEFT),
        glfw::KeyKp5 => Some(KP5),
        glfw::KeyKp6 => Some(RIGHT),
        glfw::KeyKp7 => Some(HOME),
        glfw::KeyKp8 => Some(UP),
        glfw::KeyKp9 => Some(PAGEUP),
        glfw::KeyKpDecimal => Some(COMMA),
        glfw::KeyKpDivide => Some(SLASH),
        glfw::KeyKpMultiply => Some(ASTERISK),
        glfw::KeyKpSubtract => Some(MINUS),
        glfw::KeyKpAdd => Some(PLUS),
        glfw::KeyKpEnter => Some(ENTER),
        glfw::KeyKpEqual => Some(EQUALS),
        _ => None
    }
}
