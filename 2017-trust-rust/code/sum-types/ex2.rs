
//Model a simple 'elevator' as a FSM.
//Two floor's: Ground and First.
//A single button for control: it is either
//Up or Down.
//Function next_floor should return the next floor
//given current floor and button state.

#[derive(Debug, PartialEq)]
enum Floor {
    Ground,
    First,
}

enum Button {
    Up,
    Down,
}

use Floor::*;

use Button::*;


fn next_floor(current_floor: Floor, button_pressed: Button) -> Floor {

}

fn main() {

    assert_eq!(next_floor(Ground, Down), Ground);
    assert_eq!(next_floor(Ground, Up), First);
    assert_eq!(next_floor(First, Down), Ground);
    assert_eq!(next_floor(First, Up), First);

}
