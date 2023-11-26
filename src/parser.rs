use std::error::Error;

#[allow(dead_code)]
enum SignalType {
    Plus,
    Multiple,
}
struct Signal {
    signal_type: SignalType,
    expr: usize,
    nst: usize,
}
struct Signals {
    list: Vec<Signal>,
}
impl Signals {
    fn new() -> Self {
        Signals{ list: Default::default() }
    }
    fn add(&mut self, s: Signal) {
        self.list.push(s);
    }
    #[allow(dead_code)]
    fn del(&mut self) {
        self.list.pop();
    }
    fn is_captured(&self, snapshot: &Snapshot) -> bool {
        if let Some(last) = self.list.last() {
            if last.expr != snapshot.expr || last.nst != snapshot.nst { return true };
        }
        false
    }
    fn manager(&mut self, expr: &mut Expression) -> Result<(), Box<dyn Error>> {
        let signal = self.list.last().unwrap();
        match signal.signal_type {
            SignalType::Plus => {
                let v1 = expr.expr.pop().expect("There is no expression in the stack");
                let v2 = expr.expr.pop().expect("There is no expression in the stack");
                // let num1 = v1.parse::<f64>()?;
                // let num2 = v2.parse::<f64>()?;
                if expr.nst == 0 {
                    if v1 == "x" {
                        expr.expr.push((-v2.parse::<f64>()?).to_string())
                    }
                    else if v2 == "x" {
                        expr.expr.push((-v1.parse::<f64>()?).to_string())
                    }
                    else {
                        let res = format!("{}+{}", v1.parse::<f64>()?, v2.parse::<f64>()?);
                        expr.expr.push(res);
                    };
                    self.list.pop();
                }
            },
            SignalType::Multiple => {},
        }
        Ok(())
    }
}


struct Snapshot {
    expr: usize,
    nst: usize,
}
pub struct Expression<'a> {
    value: &'a str,
    expr: Vec<String>,
    nst: usize,
}
impl<'a> Expression<'a> {
    pub fn new(value: &'a str) -> Self {
        Expression {
            value,
            expr: Default::default(),
            nst: 0,
        }
    }

    fn stack_snapshot(&self) -> Snapshot {
        let expr = self.expr.len();
        let nst = self.nst;
        Snapshot { expr, nst }
    }

    fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        // ((x)+(1))^(2*ln(x))
        // x+1

        let mut signals = Signals::new();
        let mut word = Box::new("".to_owned());

        for (i, &ch) in self.value.as_bytes().iter().enumerate() {
            let ch_str = self.value.get(i..i+1).unwrap();

            let out_snapshot = self.stack_snapshot();
            if signals.is_captured(&out_snapshot) { signals.manager(self)?; }

            match ch {
                b'(' => self.nst += 1,
                b')' => self.nst -= 1,
                b'x' => self.expr.push(ch_str.to_owned()),
                b'+' => {
                    if !word.is_empty() {
                        self.expr.push(*word.clone());
                        *word = "".to_owned();
                    }
                    let in_snapshot = self.stack_snapshot();
                    let s = Signal{
                        signal_type: SignalType::Plus,
                        expr: in_snapshot.expr,
                        nst: in_snapshot.nst,
                    };
                    signals.add(s);
                },
                b'1' => { *word += "1" },
                b'2' => { *word += "2" },
                b' ' => continue,
                _ => { return  Err(format!("There is a problem with value in the {} position ('{}')", i-1, ch_str).into()) }
            }
        }
        self.expr.push(*word);
        signals.manager(self)?;

        Ok(())
    }

    pub fn print_result(&mut self) {
        match self.parse() {
            Ok(()) => println!("Result = {:?}", self.expr),
            Err(e) => println!("{:?}", e)
        }
    }
}
