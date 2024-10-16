#[derive(Clone, Debug)]
pub(crate) struct Participant {
    pub(crate) symbol: char,
    pub(crate) colour: String,
    pub(crate) is_bot: bool,
}