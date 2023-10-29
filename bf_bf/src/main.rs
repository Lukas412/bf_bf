use bf_run::Runtime;

fn main() {
    let mut runtime = Runtime::new();

    runtime
        .set_code("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.")
        .unwrap();

    loop {
        let Some(next) = runtime.next_output() else {
            break;
        };
        print!("{}", next as char);
    }
}

struct Bruteforce {
    runtime: Runtime,
    result: String,
    state: Vec<BfState>,
}

impl Bruteforce {
    const PIECES: &'static str = ".[]+-<>";

    fn new(result: String) -> Self {
        Self {
            runtime: Runtime::new(),
            result,
            state: Vec::new(),
        }
    }

    fn find_next_vertical(&mut self, finish_execution: bool) -> String {
        todo!();
        // self.runtime.code().to_owned()
    }

    fn find_next_horizontal(&mut self, finish_execution: bool, max_length: usize) -> String {
        self.runtime.code().to_owned()
    }
}

struct BfState {
    state: Vec<BfStatePiece>,
    open_loops: usize,
    code: String,
}

impl BfState {
    fn new() -> Self {
        Self {
            state: Vec::new(),
            open_loops: 0,
            code: String::new(),
        }
    }

    fn next(&mut self) -> &str {
        let Some(last) = self.state.last() else {
            self.state.push(BfStatePiece::Print);
            return self.generate();
        };
    }
}

enum BfStatePiece {
    Print,
    LoopStart,
    Increase(usize),
    Decrease(usize),
    MoveNext,
    MovePrevious,
}
