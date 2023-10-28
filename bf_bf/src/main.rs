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
