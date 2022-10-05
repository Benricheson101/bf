mod runtime;

use runtime::Runtime;

fn main() {
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut bf = Runtime::new();
    bf.run(source).unwrap();
}
