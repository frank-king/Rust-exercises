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
    let input = b"2
8 4
-+--+--+
1 8
2 8
2 5
1 1
4 10
+-++
1 1
1 2
2 2
1 3
2 3
3 3
1 4
2 4
3 4
4 4";
    let output = b"1
2
4
4
3
3
4
2
3
2
1
2
2
2";
    verify(input, output);
}
