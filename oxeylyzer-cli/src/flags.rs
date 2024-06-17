xflags::xflags! {
    cmd oxeylyzer {
        /// Analyze a layout.
        cmd analyze a view layout {
            required name: String
        }
        /// Rank all layouts for the currently specified language. A higher score is better.
        cmd rank {}
        /// Generate <n> layouts based on a specified layout. Generates 5000 layouts by default.
        cmd gen g generate {
            required name: String
            optional -c, --count count: usize
        }
        /// Shows the top n sfbs on a layout. 10 by default.
        cmd sfbs {
            required name: String
            optional -c, --count count: usize
        }
        /// Reload the analyzer config file
        cmd r reload refresh {}
        /// Quit the analyzer
        cmd q quit {}
    }
}

//You can also specify a number to analyze a previously generated layout.
