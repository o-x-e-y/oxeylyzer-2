xflags::xflags! {
    cmd oxeylyzer {
        /// Analyze a layout.
        cmd analyze a view layout {
            /// The name of the layout to analyze.
            required name: String
        }
        /// Rank all layouts for the currently specified language. A higher score is better.
        cmd rank {}
        /// Generate layouts by randomizing a base layout.
        cmd gen g generate {
            /// Name of the layout to use as a basis.
            required name: String
            /// Name of iterations to do. 10 by default.
            optional count: usize
            /// Characters to pin. Setting pins speeds up generation.
            optional -p, --pins pins: String
        }
        /// Shows the top n sfbs on a layout.
        cmd sfbs {
            /// Name of the layout to show sfbs of.
            required name: String
            /// Amount of sfbs to show. 10 by default.
            optional -c, --count count: usize
        }
        /// Shows every trigram stat of a layout.
        cmd trigrams t {
            required name: String
        }
        /// Reload the analyzer config file
        cmd r reload refresh {}
        /// Quit the analyzer
        cmd q quit {}
    }
}

//You can also specify a number to analyze a previously generated layout.
