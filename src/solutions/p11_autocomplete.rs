// Implement an autocomplete system. That is, given a query string s and a set of all possible query strings, return all strings in the set that have s as a prefix.
//
// For example, given the query string de and the set of strings [dog, deer, deal], return [deer, deal].
//
// Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.
//
// Naïve solution:
//
// Check if the start query matches with the start of the word.
//
// Better solution: Implement a trie data structure (will do that later)

fn autocomplete<'a>(input: Vec<&'a str>, query: &'a str) -> Vec<&'a str> {
    let mut output: Vec<&str> = vec![];
    input.into_iter().for_each(|word| {
        if word.starts_with(query) {
            output.push(word)
        }
    });
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_input() {
        assert_eq!(
            autocomplete(vec!["dog", "deer", "deal"], "de"),
            vec!["deer", "deal"]
        );
    }
}
