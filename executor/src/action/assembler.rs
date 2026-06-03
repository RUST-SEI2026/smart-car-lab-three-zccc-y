use crate::action::Action;

pub(crate) trait Assembler {
    fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }

    fn move_assemble(&self) -> Vec<Action>;
    fn turn_left_assemble(&self) -> Vec<Action>;
    fn turn_right_assemble(&self) -> Vec<Action>;

    fn be_reverse(&mut self);
    fn be_fast(&mut self);
}
