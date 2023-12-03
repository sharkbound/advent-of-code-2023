pub trait JoinToStringExt<T, I, F>
where
    T: Iterator<Item = I>,
    F: Fn(I) -> String,
{
    fn join_to_string(self, sep: &str, format_item: F) -> String;
}

impl<T, I, F> JoinToStringExt<T, I, F> for T
where
    T: Iterator<Item = I>,
    F: Fn(I) -> String,
{
    fn join_to_string(self, sep: &str, format_item: F) -> String {
        let mut buf = String::new();
        let mut iterator = self.into_iter();

        if let Some(item) = iterator.next() {
            buf.push_str(&format_item(item));
        }

        for item in iterator {
            buf.push_str(sep);
            buf.push_str(&format_item(item));
        }

        buf
    }
}
