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
    let input = b"3
5 3
2 3 2 5 4
3 4
2 4 4
5 4
2 1 5 3 6";
    let output = b"NO\nYES\nYES";
    verify(input, output);
}
