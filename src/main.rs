fn main(){
    let code = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]
    <.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.".to_string();
    let mut inter = brianf::new(code);
    inter.exe();
}
struct brianf{
    input:String,
    pos:usize,
    cur_pos:usize,
    arr:[u8;3000]
}
impl brianf{
    pub fn new(input:String)->Self{
        Self{
            input:input,
            pos:0,
            cur_pos:0,
            arr:[0;3000],
        }
    }
    pub fn next(&mut self){
        self.pos += 1;
    }
    pub fn exe(&mut self){
        let mut e = 0;
        let mut is_loop = false;
        loop{
            let i = self.input.chars().nth(self.pos).unwrap_or('\'');
            if i == '\''{
                return;
            }
            match i{
                '+'=>{
                    self.arr[self.cur_pos] +=1;
                    self.next();
                },
                '-'=>{
                    self.arr[self.cur_pos] -= 1;
                    self.next();
                },
                '>'=>{
                    self.cur_pos += 1;
                    self.next();
                },
                '<'=>{
                    self.cur_pos -= 1;
                    self.next();
                },
                '.'=>{
                    print!("{}",self.arr[self.cur_pos] as char);
                    self.next();
                },
                '['=>{
                    is_loop = true;
                    e = self.pos;
                    self.next();
                }
                ']'=>{
                    if !is_loop{
                        println!("error in loop");
                        return;
                    }
                    else{
                        if self.arr[self.cur_pos] !=0{
                            self.pos = e;
                        }else{ self.next() }
                    }
                }
                _=>{
                    self.next();
                }
            }
        } 
    }
}