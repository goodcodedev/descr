#[derive(Debug)]
pub struct Hello {
}

#[derive(Debug)]
pub struct Say<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    SayItem(Say<'a>),
    HelloItem(Hello),
}

