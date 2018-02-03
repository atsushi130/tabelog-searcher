
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate tabelog_searcher;

#[cfg(test)]
mod tabelog_searcher_tests {

    use tabelog_searcher::TabelogClient;
    use tabelog_searcher::SearchCondition;

    #[test]
    fn test_tabelog_searcher() {

        let search_condition = &SearchCondition::from("渋谷", "ラーメン");
        let result = TabelogClient.search(search_condition);
        match result.is_empty() {
            true  => println!("not found."),
            false =>
                for url in result {
                    println!("{}", url);
                }
        }
    }
}
