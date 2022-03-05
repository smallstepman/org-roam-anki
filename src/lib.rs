use orgize::Org;

pub fn parse_org(content: &'static str) -> Org<'static> {
    Org::parse(content)
}
