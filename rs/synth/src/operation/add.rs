use super::super::*;

pub struct Add {
    pub inc: f32
}

impl Add {
    pub fn new(inc: f32) -> GlicolNodeData {
        mono_node!( Self { inc } )
    }
}


impl Node<128> for Add {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let max_user_input = 2;
        let min_user_input = 1;
        let l = inputs.len();
        if l < min_user_input { return ()};
        let has_clock = match l {
            0 => false,
            _ => inputs[l-1].buffers()[0][0] % 128. == 0.
            && inputs[l-1].buffers()[0][1] == 0.
        };
        // println!("l - has_clock as usize is {:?}", l - has_clock as usize);
        // println!("has_clock is {:?}",  has_clock);
        // println!(" self.inc {:?}",  self.inc);
        // println!("inputs to add node {:?}", inputs);
        match l {
            1 => {
                output[0] = inputs[0].buffers()[0].clone();
                output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);

            },
            2 => {
                if has_clock {
                    // panic!();
                    output[0] = inputs[0].buffers()[0].clone();
                    output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
                    // println!("output[0] should be {:?}", output[0]);
                } else {
                    let buf = &mut inputs[0].buffers();
                    let mod_buf = &mut inputs[1].buffers();
                    for i in 0..128 {
                        output[0][i] = mod_buf[0][i] + buf[0][i];
                    }
                }
            },
            3 => {
                let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[1].buffers();
                for i in 0..128 {
                    output[0][i] = mod_buf[0][i] + buf[0][i];
                }
            },
            _ => return ()
        };
    }
}

#[macro_export]
macro_rules! add {
    () => {
        Add::new(0.0)
    };

    ($data: expr) => {
        Add::new($data)
    };
}
