extern crate termion;

pub mod renderer {
    use termion::{color, style,raw::IntoRawMode};
    use std::io::{Write, stdout};
    use crate::Game;


    
pub struct Renderer{
    playable_square : char,
    unplayable_square: char,
    white_piece: char,
    unwhite_piece: char,
    origin_x: u16,
    origin_y: u16,
    info_x: u16,
    info_y: u16,
    prompt_x: u16,
    prompt_y: u16
}

impl Renderer{
    
    pub fn new()->Renderer{
        Renderer{
            playable_square: '#',
            unplayable_square: '#',
            white_piece: '0',
            unwhite_piece: '0',
            origin_x: 12,
            origin_y: 12,
            info_x: 8,
            info_y: 14,
            prompt_x: 8,
            prompt_y:22
        }
    }

    pub fn draw(&self, game:&Game){
        
        let mut stdout = stdout().into_raw_mode().unwrap();

        print!("{}", termion::clear::All);

        for y in 0..8{
            for x in 0..8{
                if (y%2==0 && x%2==0) || (y%2!=0 && x%2!=0) {
                    print!("{}{}{}", termion::cursor::Goto(self.origin_x-x,
                                                           self.origin_y-y), 
                           color::Fg(color::Yellow),self.playable_square);
                    
                }
                else {
                    print!("{}{}{}", termion::cursor::Goto(self.origin_x -x,
                                                            self.origin_y-y), 
                           color::Fg(color::Red),self.unplayable_square);
                    

                }

            }
            
        }

        // draw pieces
        for i in 0..24 {
            let dx:u16 = game.pieces[i].loc_x;
            let dy:u16 = game.pieces[i].loc_y;
            let is_white:bool = game.pieces[i].is_white;
            
            if is_white {
                {print!("{}{}{}",
                   termion::cursor::Goto(u16::from(self.origin_y-dy),
                   u16::from(self.origin_x-dx)),
                   color::Fg(color::White),
                   self.white_piece
                   );
                }
            }
            else{
                print!("{}{}{}",
                   termion::cursor::Goto(u16::from(self.origin_y-dy),
                   u16::from(self.origin_x-dx)),
                   color::Fg(color::Black),
                   self.unwhite_piece
                   );
             
            }


        }

       

    }

    pub fn print_info(&self, message:String){
        println!("{}{}{}",termion::cursor::Goto(
                self.info_x,self.info_y),
                color::Fg(color::White),
                message);

        print!("{}",termion::cursor::Goto(self.prompt_x, self.prompt_y));

    }
}


}