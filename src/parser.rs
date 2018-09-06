use regex::{Regex};

pub struct Parser {
    name_regex: Regex,
}

impl Parser {
    pub fn new() -> Self {
        let name_regex = Regex::new(r#"<div class="player">\s*<a.*>(?P<name>.*)</a>\s*</div>"#).unwrap();

        Parser {
            name_regex,
        }
    }

    pub fn parse(&self, text: &str) -> Vec<Player> {
        let name_captures = self.name_regex.captures_iter(text);

        name_captures
            .map(|captures| captures
                .name("name")
                .and_then(|result| Some(result.as_str()))
                .map(|player_name|
                    Player { name: player_name.to_string() }
                )
            )
            .filter_map(|p| p)
            .collect::<Vec<Player>>()
    }
}

pub struct Player {
    name: String,
}

#[cfg(test)]
mod test {
    use super::*;

    const HTML_FIXTURE: &'static str = r#"
<tr class="top3 ">
    <td>1</td>
    <td>
        <div class="lb-profile">
            <div class="player">
                <a style="color:#f17f1a;" href="/bf1/profile/pc/AuMreaching7King">AuMreaching7King</a>
            </div>
            <div class="icons">
                <img class="flag" src="/images/flags/China.png" />
            </div>
        </div>
    </td>
    <td>
        <div class="pull-left">
            <small class="text-success">Top 1%</small>
        </div>
        <div class="pull-right">
            373,615 </div>
    </td>
    <td class="right">13520</td>
</tr>
<tr class="top3 ">
    <td>2</td>
    <td>
        <div class="lb-profile">
            <div class="player">
                <a style="color:#f17f1a;" href="/bf1/profile/pc/Silmary_Fr">Silmary_Fr</a>
            </div>
            <div class="icons">
                <img class="flag" src="/images/flags/France.png" />
            </div>
        </div>
    </td>
</tr>
    "#;

    const EXPECTED_NAMES: [&str; 2] = ["AuMreaching7King", "Silmary_Fr"];

    #[test]
    fn it_fetches_the_player_name() {
        let parser = Parser::new();
        let result = parser.parse(HTML_FIXTURE);

        result
            .iter()
            .zip(EXPECTED_NAMES.iter())
            .for_each(|(actual, expected)|
                assert_eq!(actual.name, *expected)
            );
    }
}