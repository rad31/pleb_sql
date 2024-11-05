use super::ScriptIterator;

#[test]
fn test_line_and_position() {
    let input = "012\t\r\n\
                 012\n\
                 012";

    let mut iterator = ScriptIterator::new(input);

    let (_, c) = iterator.peek().unwrap();
    assert_eq!(*c, '0');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 0);

    let (_, c) = iterator.peek().unwrap();
    assert_eq!(*c, '0');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 0);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '0');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 1);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '1');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 2);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '2');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 3);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '\t');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 4);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '\r');
    assert_eq!(iterator.line, 0);
    assert_eq!(iterator.index, 5);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '\n');
    assert_eq!(iterator.line, 1);
    assert_eq!(iterator.index, 0);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '0');
    assert_eq!(iterator.line, 1);
    assert_eq!(iterator.index, 1);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '1');
    assert_eq!(iterator.line, 1);
    assert_eq!(iterator.index, 2);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '2');
    assert_eq!(iterator.line, 1);
    assert_eq!(iterator.index, 3);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '\n');
    assert_eq!(iterator.line, 2);
    assert_eq!(iterator.index, 0);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '0');
    assert_eq!(iterator.line, 2);
    assert_eq!(iterator.index, 1);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '1');
    assert_eq!(iterator.line, 2);
    assert_eq!(iterator.index, 2);

    let (_, c) = iterator.next().unwrap();
    assert_eq!(c, '2');
    assert_eq!(iterator.line, 2);
    assert_eq!(iterator.index, 3);

    let end = iterator.peek();
    assert!(end.is_none());
    let end = iterator.next();
    assert!(end.is_none());
}
