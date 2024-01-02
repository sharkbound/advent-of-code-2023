#[macro_export]
/// splits a [str] or [String] into a [Vec] of [&str]
///
/// # Usage:
/// ## spliting by whitespace:
/// ```
/// use rustutils::str_split;
/// let string = "hello, world";
/// // ["hello,", "world"]
/// let split = str_split!(string);
/// ```
/// ## spliting by custom delimiter:
/// ```
/// use rustutils::str_split;
/// let string = "hello, world";
/// // ["hello", " world"]
/// let split = str_split!(string, ",");
/// ```
macro_rules! str_split {
    ($str:expr) => {
        $str.split_whitespace().collect::<Vec<&str>>()
    };

    ($str:expr, $sep:expr) => {
        $str.split($sep).collect::<Vec<&str>>()
    };
}

#[macro_export]
/// splits a [str] or [String] into a [Vec] of [String]
///
/// # Usage:
/// ## spliting by whitespace:
/// ```
/// use rustutils::string_split;
/// let string = "hello, world";
/// // ["hello,", "world"]
/// let split = string_split!(string);
/// ```
/// ## spliting by custom delimiter:
/// ```
/// use rustutils::string_split;
/// let string = "hello, world";
/// // ["hello", " world"]
/// let split = string_split!(string, ",");
/// ```
macro_rules! string_split {
    // string_split!(items);
    ($string:expr) => {
        $string
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };

    // string_split!(items, " ");
    ($string:expr, $sep:expr) => {
        $string
            .split($sep)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };
}

#[macro_export]
macro_rules! join_to_string_debug {
    ($iterator:expr, $sep:expr) => {{
        let mut buf = String::new();
        let mut iterator = $iterator.into_iter();

        if let Some(item) = iterator.next() {
            buf.push_str(&format!("{:?}", item));
        }

        for item in iterator {
            buf.push_str($sep);
            buf.push_str(&format!("{:?}", item));
        }

        buf
    }};

    ($iterator:expr) => {
        join_to_string_debug!($iterator, ", ")
    }
}

#[macro_export]
macro_rules! join_to_string_debug_expanded {
    ($iterator:expr, $sep:expr) => {{
        let mut buf = String::new();
        let mut iterator = $iterator.into_iter();

        if let Some(item) = iterator.next() {
            buf.push_str(&format!("{:#?}", item));
        }

        for item in iterator {
            buf.push_str($sep);
            buf.push_str(&format!("{:#?}", item));
        }

        buf
    }};

    ($iterator:expr) => {
        join_to_string_debug_expanded!($iterator, ", ")
    }
}

#[macro_export]
macro_rules! join_to_string {
    ($iterator:expr, $sep:expr) => {{
        let mut buf = String::new();
        let mut iterator = $iterator.into_iter();

        if let Some(item) = iterator.next() {
            buf.push_str(&format!("{}", item));
        }

        for item in iterator {
            buf.push_str($sep);
            buf.push_str(&format!("{}", item));
        }

        buf
    }};

    ($iterator:expr) => {
        join_to_string!($iterator, ", ")
    }
}