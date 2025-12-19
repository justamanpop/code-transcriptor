#[cfg(test)]
mod tests {
    use super::super::clean_transcript;

    //This is tested because transcript returned by whisper has spaces between the word
    //double quotes and the actual string itself. Clean should remove that
    #[test]
    fn test_clean_transcript_single_line_one_string_with_leading_and_trailing_space_assignment() -> Result<(), String> {
        //arrange
        let input = String::from("a colon equals \" hello \"");

        //act
        let actual = clean_transcript(input);

        //assert
        let expected = "a := \"hello\"\n";
        assert_eq!(expected, actual);
        Ok(())
    }

    //If "space" is found, it should be replaced with " "
    #[test]
    fn test_clean_transcript_word_space_inside_double_quotes() -> Result<(), String> {
        //arrange
        let input = String::from("a colon equals \" space \"");

        //act
        let actual = clean_transcript(input);

        //assert
        let expected = "a := \" \"\n";
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_clean_transcript_concatenated_string_assignment() -> Result<(), String> {
        //arrange
        let input = String::from("a colon equals \" hello \" plus \" world \"");

        //act
        let actual = clean_transcript(input);

        //assert
        let expected = "a := \"hello\" + \"world\"\n";
        assert_eq!(expected, actual);
        Ok(())
    }

    // #[test]
    // fn test_clean_transcript_multi_line_string_assignment() -> Result<(), String> {
    //     //arrange
    //     let input = String::from("a colon equals \" hello \" b colon equals \" world \"");
    //
    //     //act
    //     let actual = clean_transcript(input);
    //
    //     //assert
    //     let expected = vec!["a := \"hello\"", " b := \"world\"",""].join("\n");
    //     assert_eq!(expected, actual);
    //     Ok(())
    // }

    #[test]
    fn test_clean_transcript_multi_line_string_assignment() -> Result<(), String> {
        //arrange
        let input = String::from("a colon equals \" hello \" b colon equals \" hello \" plus \" space \" plus \" world \" c colon equals \" ho \" plus \" ho \"");

        //act
        let actual = clean_transcript(input);

        //assert
        let expected = vec!["a := \"hello\"", " b := \"hello\" + \" \" + \"world\"", " c := \"ho\" + \"ho\"", ""].join("\n");
        assert_eq!(expected, actual);
        Ok(())
    }
}
