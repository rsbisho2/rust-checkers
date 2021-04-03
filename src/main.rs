extern crate termion;
use termion::{color, style,raw::IntoRawMode};
use std::io::{Write, stdout};


struct Game {
    pieces: [Piece;24]
}

#[derive(Debug,Copy,Clone)]
struct Piece{
    loc_x: u8,
    loc_y: u8,
    is_active: bool,
    is_kinged: bool,
    is_white: bool
}

impl Game {
    fn new()->Game{

        let mut pieces_arr = [Piece::new(0,0);24];
        
        //todo dear god why
        pieces_arr[0].initialize(0,0,false);
        pieces_arr[1].initialize(0,2,false);
        pieces_arr[2].initialize(0,4,false);
        pieces_arr[3].initialize(0,6,false);
        pieces_arr[4].initialize(1,1,false);
        pieces_arr[5].initialize(1,3,false);
        pieces_arr[6].initialize(1,5,false);
        pieces_arr[7].initialize(1,7,false);
        pieces_arr[8].initialize(2,0,false);
        pieces_arr[9].initialize(2,2,false);
        pieces_arr[10].initialize(2,4,false);
        pieces_arr[11].initialize(2,6,false);

        pieces_arr[12].initialize(5,1,true);
        pieces_arr[13].initialize(5,3,true);
        pieces_arr[14].initialize(5,5,true);
        pieces_arr[15].initialize(5,7,true);
        pieces_arr[16].initialize(6,0,true);
        pieces_arr[17].initialize(6,2,true);
        pieces_arr[18].initialize(6,4,true);
        pieces_arr[19].initialize(6,6,true);
        pieces_arr[20].initialize(7,1,true);
        pieces_arr[21].initialize(7,3,true);
        pieces_arr[22].initialize(7,5,true);
        pieces_arr[23].initialize(7,7,true);

        return Game{pieces:pieces_arr}
    }

}

impl Piece{
    fn new(x_ : u8, y_ : u8)->Piece{

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

    fn move_piece(&mut self, new_x:u8, new_y:u8){
        self.loc_x = new_x;
        self.loc_y = new_y;
    }

    fn initialize(&mut self, new_x:u8, new_y:u8, is_white_:bool){
        self.loc_x = new_x;
        self.loc_y = new_y;
        self.is_white = is_white_;
    }

   

}

struct Renderer{
    playable_square : char,
    unplayable_square: char,
    white_piece: char,
    unwhite_piece:char
}

impl Renderer{
    
    fn new()->Renderer{
        Renderer{
            playable_square: '#',
            unplayable_square: '#',
            white_piece: '8',
            unwhite_piece: '0'
        }
    }

    fn draw(&self, game:Game){
        
        let mut stdout = stdout().into_raw_mode().unwrap();

        // clear screen
        //print!("{}[2J", 27 as char);

        print!("{}", termion::clear::All);

        for y in 0..8{
            for x in 0..8{
                if (y%2==0 && x%2==0) || (y%2!=0 && x%2!=0) {
                    print!("{}{}{}", termion::cursor::Goto(10-x,10-y), 
                           color::Fg(color::Black),self.playable_square);
                    
                }
                else {
                    print!("{}{}{}", termion::cursor::Goto(10-x,10-y), 
                           color::Fg(color::White),self.unplayable_square);
                    

                }

            }
            
        }

        // draw pieces
        for i in 0..23 {
            let dx:u8 = game.pieces[i].loc_x;
            let dy:u8 = game.pieces[i].loc_y;
            let is_white:bool = game.pieces[i].is_white;

            print!("{}{}O",
                   termion::cursor::Goto(u16::from(10-dy),
                   u16::from(10-dx)),
                   color::Fg(color::Red)
                   );

        }

        println!("{}",termion::cursor::Goto(1,30));


    }
}

fn main() {
    println!("Starting Checkers!");

    let game = Game::new();

    for i in 0..23{
        println!("{}", game.pieces[i].to_string());
    }

    let renderer = Renderer::new();
    renderer.draw(game);
}

