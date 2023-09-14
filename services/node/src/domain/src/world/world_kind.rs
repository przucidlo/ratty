use std::str::FromStr;

#[derive(Debug)]
pub enum WorldKind {
    Public,
    Private,
}

impl FromStr for WorldKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => Err(()),
        }
    }
}

impl ToString for WorldKind {
    fn to_string(&self) -> String {
        match self {
            WorldKind::Public => "public".to_owned(),
            WorldKind::Private => "private".to_owned(),
        }
    }
}
