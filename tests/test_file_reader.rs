use xml_reader::file_reader::*;

#[test]
fn test_text(){
    let text : String ="<note>".to_string();
    let start = text.find('<').unwrap();
    let end = text.find('>').unwrap()+1;
    println!("{}", text[start..end].to_string());
}

#[test]
fn test_line_formatter(){
    let mut extracted_line_1 = vec![
        "<book>".to_string(),
        "<title lang=\"en\">".to_string(),
        "Harry Potter".to_string(),
        "</title>".to_string(),
        "<meta/>".to_string(),
        "</book>".to_string()
    ];

    let mut extracted_line_2 = vec![
        "<note>".to_string(),
        "<to>".to_string(),
        "Tove".to_string(),
        "</to>".to_string(),
        "<from>".to_string(),
        "Jani ".to_string(),
        "<bold>".to_string(),
        "The duck".to_string(),
        "</bold>".to_string(),
        "</from>".to_string(),
        "</note>".to_string()
    ];

    let expected_output_1 = vec![
        "<book>".to_string(),
        "\t<title lang=\"en\">".to_string(),
        "\t\tHarry Potter".to_string(),
        "\t</title>".to_string(),
        "\t<meta/>".to_string(),
        "</book>".to_string()
    ];

    let expected_output_2 =  vec![
        "<note>".to_string(),
        "\t<to>".to_string(),
        "\t\tTove".to_string(),
        "\t</to>".to_string(),
        "\t<from>".to_string(),
        "\t\tJani ".to_string(),
        "\t\t<bold>".to_string(),
        "\t\t\tThe duck".to_string(),
        "\t\t</bold>".to_string(),
        "\t</from>".to_string(),
        "</note>".to_string()
    ];

    let output_1 = line_formatter(0, extracted_line_1);
    let output_2 = line_formatter(0, extracted_line_2);

    for (i, (actual, expected)) in output_1.iter().zip(expected_output_1.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

    for (i, (actual, expected)) in output_2.iter().zip(expected_output_2.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

}

#[test]
fn test_line_extract(){
    let mut line_1 = "<note><to>Tove</to><from>Jani <bold>The duck</bold></from></note>".to_string();
    let mut line_2 = "<book><title lang=\"en\">Harry Potter</title><meta/></book>".to_string();

    let expected_output_1 = vec![
        "<note>".to_string(),
        "<to>".to_string(),
        "Tove".to_string(),
        "</to>".to_string(),
        "<from>".to_string(),
        "Jani ".to_string(),
        "<bold>".to_string(),
        "The duck".to_string(),
        "</bold>".to_string(),
        "</from>".to_string(),
        "</note>".to_string()
    ];

    let expected_output_2 = vec![
        "<book>".to_string(),
        "<title lang=\"en\">".to_string(),
        "Harry Potter".to_string(),
        "</title>".to_string(),
        "<meta/>".to_string(),
        "</book>".to_string()
    ];

    let new_format_1 : Vec<String> = line_extractor(&mut line_1);
    let new_format_2 : Vec<String> = line_extractor(&mut line_2);

    for (i, (actual, expected)) in new_format_1.iter().zip(expected_output_1.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

    for (i, (actual, expected)) in new_format_2.iter().zip(expected_output_2.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

}

#[test]
fn test_file_formatter(){
    let simulated_file_1 = read_xml_file("tests\\to_format_example_1.xml").unwrap();
    let simulated_file_2 = read_xml_file("tests\\to_format_example_2.xml").unwrap();

    let result_1 : Vec<String> = vec![
        "<head>".to_string(),
        "\t<body>".to_string(),
        "\t\t<title lang=\"en\">".to_string(),
        "\t\t\t<p>".to_string(),
        "\t\t\t\tThis is a ".to_string(),
        "\t\t\t\t<bold>".to_string(),
        "\t\t\t\t\tFormatted File?".to_string(),
        "\t\t\t\t</bold>".to_string(),
        "\t\t\t</p>".to_string(),
        "\t\t\t<br/>".to_string(),
        "\t\t</title>".to_string(),
        "\t</body>".to_string(),
        "</head>".to_string()
    ];

    let result_2 : Vec<String> = vec![
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string(),
        "<!-- This is a test XML file with edge cases -->".to_string(),
        "<head>".to_string(),
        "\t<meta charset=\"UTF-8\" />".to_string(),
        "\t<body>".to_string(),
        "\t\t<title lang=\"en\">".to_string(),
        "\t\t\t<p>".to_string(),
        "\t\t\t\tThis is a ".to_string(),
        "\t\t\t\t<bold>".to_string(),
        "\t\t\t\t\tFormatted File?".to_string(),
        "\t\t\t\t</bold>".to_string(),
        "\t\t\t</p>".to_string(),
        "\t\t\t<br/>".to_string(),
        "\t\t\t<p>".to_string(),
        "\t\t\t\tNested ".to_string(),
        "\t\t\t\t<italic>".to_string(),
        "\t\t\t\t\tcontent".to_string(),
        "\t\t\t\t</italic>".to_string(),
        "\t\t\t\t with attributes.".to_string(),
        "\t\t\t</p>".to_string(),
        "\t\t\t<p>".to_string(),
        "\t\t\t\t<link href=\"https://example.com\" />".to_string(),
        "\t\t\t</p>".to_string(),
        "\t\t</title>".to_string(),
        "\t\t<footer>".to_string(),
        "\t\t\tSome text content in the footer.".to_string(),
        "\t\t</footer>".to_string(),
        "\t</body>".to_string(),
        "</head>".to_string(),
    ];

    let resulting_line_1 = file_formatter(&simulated_file_1);
    let resulting_line_2 = file_formatter(&simulated_file_2);

    for (i, (actual, expected)) in resulting_line_1.iter().zip(result_1.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

    for (i, (actual, expected)) in resulting_line_2.iter().zip(result_2.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }

}