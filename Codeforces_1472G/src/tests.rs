use crate::Scanner;

fn trim(text: String) -> String {
    text.split('\n').map(|line| line.trim()).fold(
        String::new(), |mut output, line| {
            output.push_str(line);
            output.push('\n');
            output
        }).trim_end().to_string()
}

fn verify(input: &[u8], output: &[u8]) {
    use crate::run;

    let input = Scanner::from(&input[..]);
    let mut answer = Vec::new();
    run(input, &mut answer).unwrap();

    let output = String::from_utf8_lossy(output).to_string();
    let answer = String::from_utf8(answer).unwrap();
    assert_eq!(trim(output), trim(answer));
}

#[test]
fn test() {
    let input = b"3

6 7
1 2
1 3
2 5
2 4
5 1
3 6
6 2

2 2
1 2
2 1

6 8
1 2
1 5
2 6
6 1
2 3
3 4
4 2
5 4";

    let output =
    b"0 0 1 2 0 1
0 0
0 0 2 1 1 0";
    verify(input, output);
}
