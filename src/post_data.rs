use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryFormat<'a> {
    pub filters: FilterItems,
    pub fields: &'a str,
    pub sort: &'a str,
    pub reverse: bool,
    pub results: usize,
    pub page: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
    pub count: bool,
    pub compact_filters: bool,
    pub normalized_filters: bool,
}

impl<'a> QueryFormat<'a> {
    pub fn builder() -> QueryFormatBuilder<'a> {
        QueryFormatBuilder::new()
    }
}

pub struct QueryFormatBuilder<'a> {
    pub filters: FilterItems,
    pub fields: &'a str,
    pub sort: &'a str,
    pub reverse: bool,
    pub results: usize,
    pub page: usize,
    pub user: Option<&'a str>,
    pub count: bool,
    pub compact_filters: bool,
    pub normalized_filters: bool,
}

impl<'a> QueryFormatBuilder<'a> {
    fn new() -> Self {
        Self {
            filters: Vec::new(),
            fields: "",
            sort: "id",
            reverse: false,
            results: 10,
            page: 1,
            user: None,
            count: false,
            compact_filters: false,
            normalized_filters: false,
        }
    }

    pub fn filters(mut self, val: FilterItems) -> Self {
        self.filters = val;
        self
    }

    pub fn fields(mut self, val: &'a str) -> Self {
        self.fields = val;
        self
    }

    pub fn sort(mut self, val: &'a str) -> Self {
        self.sort = val;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    pub fn results(mut self, val: usize) -> Self {
        self.results = val;
        self
    }

    pub fn page(mut self, val: usize) -> Self {
        self.page = val;
        self
    }

    pub fn user(mut self, val: &'a str) -> Self {
        self.user = Some(val);
        self
    }

    pub fn count(mut self) -> Self {
        self.count = true;
        self
    }

    pub fn compact_filters(mut self) -> Self {
        self.compact_filters = true;
        self
    }

    pub fn normalized_filters(mut self) -> Self {
        self.normalized_filters = true;
        self
    }

    pub fn build(self) -> QueryFormat<'a> {
        QueryFormat {
            filters: self.filters,
            fields: self.fields,
            sort: self.sort,
            reverse: self.reverse,
            results: self.results,
            page: self.page,
            user: self.user,
            count: self.count,
            compact_filters: self.compact_filters,
            normalized_filters: self.normalized_filters,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    pub results: serde_json::Value,
    pub more: bool,
    pub count: Option<usize>,
    pub compact_filters: Option<String>,
    pub normalized_filters: Option<FilterItems>,
}

pub type FilterItems = Vec<FilterItem>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum FilterItem {
    S(String),
    V(FilterItems),
}

impl FilterItem {
    pub fn from_json(json: serde_json::Value) -> Self {
        serde_json::from_value(json).unwrap()
    }
}

impl From<FilterItems> for FilterItem {
    fn from(value: FilterItems) -> Self {
        Self::V(value)
    }
}

impl From<String> for FilterItem {
    fn from(value: String) -> Self {
        Self::S(value)
    }
}

impl From<&str> for FilterItem {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl<T, const N: usize> From<[T; N]> for FilterItem
where
    T: Into<FilterItem> + Clone,
{
    fn from(value: [T; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T> From<&[T]> for FilterItem
where
    T: Into<FilterItem> + Clone,
{
    fn from(value: &[T]) -> Self {
        Self::from(
            value
                .iter()
                .cloned()
                .map(|x| x.into())
                .collect::<FilterItems>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FilterItem;

    #[test]
    fn simple_string_vec() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(vec!["abc".into(), "=".into()]);
        assert_eq!(test_item, ans);
    }

    #[test]
    fn simple_string_array() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(["abc", "="]);
        assert_eq!(test_item, ans);
    }

    #[test]
    fn simple_string_slice() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(&["abc", "="][..]);
        assert_eq!(test_item, ans);
    }

    #[test]
    fn complex_macro_test() {
        let ans = FilterItem::V(vec![
            FilterItem::S("and".to_string()),
            FilterItem::V(vec![
                FilterItem::S("or".to_string()),
                FilterItem::V(vec![
                    FilterItem::S("lang".to_string()),
                    FilterItem::S("=".to_string()),
                    FilterItem::S("en".to_string()),
                ]),
                FilterItem::V(vec![
                    FilterItem::S("lang".to_string()),
                    FilterItem::S("=".to_string()),
                    FilterItem::S("de".to_string()),
                ]),
                FilterItem::V(vec![
                    FilterItem::S("lang".to_string()),
                    FilterItem::S("=".to_string()),
                    FilterItem::S("fr".to_string()),
                ]),
            ]),
            FilterItem::V(vec![
                FilterItem::S("olang".to_string()),
                FilterItem::S("!=".to_string()),
                FilterItem::S("ja".to_string()),
            ]),
            FilterItem::V(vec![
                FilterItem::S("release".to_string()),
                FilterItem::S("=".to_string()),
                FilterItem::V(vec![
                    FilterItem::S("and".to_string()),
                    FilterItem::V(vec![
                        FilterItem::S("released".to_string()),
                        FilterItem::S(">=".to_string()),
                        FilterItem::S("2020-01-01".to_string()),
                    ]),
                    FilterItem::V(vec![
                        FilterItem::S("producer".to_string()),
                        FilterItem::S("=".to_string()),
                        FilterItem::V(vec![
                            FilterItem::S("id".to_string()),
                            FilterItem::S("=".to_string()),
                            FilterItem::S("p30".to_string()),
                        ]),
                    ]),
                ]),
            ]),
        ]);
        let temp = serde_json::json!([
            "and",
            [
                "or",
                ["lang", "=", "en"],
                ["lang", "=", "de"],
                ["lang", "=", "fr"]
            ],
            ["olang", "!=", "ja"],
            [
                "release",
                "=",
                [
                    "and",
                    ["released", ">=", "2020-01-01"],
                    ["producer", "=", ["id", "=", "p30"]]
                ]
            ]
        ]);
        let temp = FilterItem::from_json(temp);
        assert_eq!(temp, ans);
    }
}
