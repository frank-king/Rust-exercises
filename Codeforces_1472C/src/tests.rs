use crate::Scanner;

fn trim(text: String) -> String {
    text.split('\n')
        .map(|line| line.trim())
        .fold(String::new(), |mut output, line| {
            output.push_str(line);
            output.push('\n');
            output
        })
        .trim_end()
        .to_string()
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
    let input = b"4
5
7 3 1 2 3
3
2 1 4
6
2 1000 2 3 995 1
5
1 1 1 1 1";
    let output = b"7\n6\n1000\n5";
    verify(input, output);
}
