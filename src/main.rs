extern crate termion;
mod renderer;
use renderer::renderer::Renderer;
use termion::{color, style,raw::IntoRawMode};
use std::io::{Write, stdout};
//pub use crate::renderer::renderer;

struct Game {
    pieces: [Piece;24],
    active: bool,
    white_turn:bool
}

#[derive(Debug,Copy,Clone)]
struct Piece{
    loc_x: u16,
    loc_y: u16,
    is_active: bool,
    is_kinged: bool,
    is_white: bool
}

impl Game {
    fn new()->Game{

        let mut pieces_arr = [Piece::new(0,0);24];
        
        //todo dear god why
        pieces_arr[0].initialize(0,0,true);
        pieces_arr[1].initialize(0,2,true);
        pieces_arr[2].initialize(0,4,true);
        pieces_arr[3].initialize(0,6,true);
        pieces_arr[4].initialize(1,1,true);
        pieces_arr[5].initialize(1,3,true);
        pieces_arr[6].initialize(1,5,true);
        pieces_arr[7].initialize(1,7,true);
        pieces_arr[8].initialize(2,0,true);
        pieces_arr[9].initialize(2,2,true);
        pieces_arr[10].initialize(2,4,true);
        pieces_arr[11].initialize(2,6,true);

        pieces_arr[12].initialize(5,1,false);
        pieces_arr[13].initialize(5,3,false);
        pieces_arr[14].initialize(5,5,false);
        pieces_arr[15].initialize(5,7,false);
        pieces_arr[16].initialize(6,0,false);
        pieces_arr[17].initialize(6,2,false);
        pieces_arr[18].initialize(6,4,false);
        pieces_arr[19].initialize(6,6,false);
        pieces_arr[20].initialize(7,1,false);
        pieces_arr[21].initialize(7,3,false);
        pieces_arr[22].initialize(7,5,false);
        pieces_arr[23].initialize(7,7,false);

        return Game{pieces:pieces_arr, active:true, white_turn:true}
    }
    fn run(&mut self, renderer:Renderer){
        renderer.draw(self);
        while self.active{
            if self.white_turn {
                renderer.print_info("It's white's turn:".to_string());
                

                self.white_turn = false;
            }
            else {
                renderer.print_info("It's black's turn:".to_string());

                self.white_turn = true;
            }
        }
    }

}

impl Piece{
    fn new(x_ : u16, y_ : u16)->Piece{

        return Piece{
            loc_x: x_,
            loc_y: y_,
            is_white : if y_ > 4 {true} else {false},
            is_active: true,
            is_kinged : false
        }
    }

    fn to_string(&self) -> String {
        return format!("x: {}, y: {}, is_white: {}", self.loc_x, 
                            self.loc_y, self.is_white);
    }

    fn move_piece(&mut self, new_x:u16, new_y:u16){
        self.loc_x = new_x;
        self.loc_y = new_y;
    }

    fn initialize(&mut self, new_x:u16, new_y:u16, is_white_:bool){
        self.loc_x = new_x;
        self.loc_y = new_y;
        self.is_white = is_white_;
    }

   

}

fn main() {
    println!("Starting Checkers!");

    let mut game = Game::new();
    let renderer = Renderer::new();
    //renderer.draw(game);
    renderer.print_info(String::from("New game started.  White to move."));
    
    game.run(renderer);

    println!("{}Goodbye!",termion::cursor::Goto(1,30));
}

