use scraper::{ElementRef, Html, Selector};

pub trait Query {
    fn query_selector(&self, selector: &str) -> ElementRef<'_>;
    fn query_selector_all(&self, selector: &str) -> Vec<ElementRef<'_>>;
}

impl Query for Html {
    fn query_selector(&self, selector: &str) -> ElementRef<'_> {
        self.select(&Selector::parse(selector).unwrap())
            .into_iter()
            .map(|item| item)
            .collect::<Vec<ElementRef>>()[0]
    }

    fn query_selector_all(&self, selector: &str) -> Vec<ElementRef<'_>> {
        self.select(&Selector::parse(selector).unwrap())
            .into_iter()
            .map(|item| item)
            .collect::<Vec<ElementRef>>()
    }
}

impl Query for ElementRef<'_> {
    fn query_selector(&self, selector: &str) -> ElementRef<'_> {
        self.select(&Selector::parse(selector).unwrap())
            .into_iter()
            .map(|item| item)
            .collect::<Vec<ElementRef>>()[0]
    }

    fn query_selector_all(&self, selector: &str) -> Vec<ElementRef<'_>> {
        self.select(&Selector::parse(selector).unwrap())
            .into_iter()
            .map(|item| item)
            .collect::<Vec<ElementRef>>()
    }
}
