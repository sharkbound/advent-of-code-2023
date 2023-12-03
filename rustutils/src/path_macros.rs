#[macro_export]
macro_rules! path {
   ($($x:expr),+ $(,)?) => {{
       let mut path = std::path::PathBuf::new();
       $(path.push($x);)+
       path
   }};

    ($path:expr) => {{
        let mut path = std::path::PathBuf::new();
        path.push($path);
        path
    }};
}

#[macro_export]
macro_rules! path_from_root {
   ($root:expr, $($x:expr),+ $(,)?) => {{
       let mut path = $root.clone();
       $(path.push($x);)+
       path
   }};

    ($root:expr, $child:expr) => {{
       let mut path = $root.clone();
       path.push($child);
       path
   }};
}

#[macro_export]
macro_rules! path_from_str {
   ($root:expr, $($x:expr),+ $(,)?) => {{
       let mut path = std::path::PathBuf::from($root);
       $(path.push($x);)+
       path
   }};

    ($root:expr) => {
        std::path::PathBuf::from($root)
   };
}
