use pyo3::prelude::*;
use aho_corasick::AhoCorasick;

#[pyclass]
struct AhoCClass {}

#[pymethods]
impl AhoCClass {
    #[new]
    pub fn __new__() -> Self {
        AhoCClass {}
    }

    pub fn test_array(&self, wordlist: Vec<&str>) -> PyResult<String> {
        Ok(format!("hello there {}", wordlist.len()))
    }

    pub fn test_arraycounts(&self, wordlist: Vec<&str>) -> PyResult<Vec<usize>> {
        let mut result1 = Vec::new();
        result1.push(wordlist.len());
        result1.push(wordlist.len()*3);
        Ok(result1)
    }


    pub fn test(&self, haystack: &str) -> PyResult<String> {

        let patterns = &["apple", "maple", "Snapple"];
        // let haystack = "Nobody likes maple in their apple flavored Snapple.";

        let ac = AhoCorasick::new(patterns);
        let mut matches = vec![];
        for mat in ac.find_iter(haystack) {
            matches.push((mat.pattern(), mat.start(), mat.end()));
        }
        assert_eq!(matches, vec![
            (1, 13, 18),
            (0, 28, 33),
            (2, 43, 50),
        ]);

        Ok(format!("hello there {}", haystack))
    }

    pub fn greeting(&self) -> &'static str {
        "Hello, world!"
    }
}

#[pymodule]
pub fn ahoc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AhoCClass>()?;
    Ok(())
}