
use crate::view::input_view;
use crate::view::output_view;


pub struct GameView {
    input:  Box<dyn input_view::Input>,
    output: Box<dyn output_view::Output>

}

impl GameView { 

    pub fn start(&self){
        self.output.println("숫자 야구게임을 시작합니다");
    }
}
