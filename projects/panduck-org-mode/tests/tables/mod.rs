use panduck_org_mode::MarkdownParser;

const TEST_TABLE1: &'static str = r#"
# git hash: f918bf4
# build time: Mon, 01 Apr 2024 03:44:43 +0000

*Orgize*, a /pure/ =Rust= library for
parsing +Emacs+ _org-mode_ files.

See also:
[[https://github.com/PoiScript/orgize][GitHub]] |
[[https://crates.io/crates/orgize][crates.io]] |
[[https://www.npmjs.com/package/orgize][NPM]]

-----

* Heading 1 *bold*
** Heading 2 =verbatim=
*** Heading 3 ~code~
**** Heading 4 /italic/
***** Heading 5 +strike+
****** Heading 6 _underline_

This's section

#+begin_quote
This is a quote
#+end_quote

#+begin_example
This is an example block
#+end_example

-----
List

1. First item
2. Second item
3. Third item
    * Indented item
    * Indented item
4. Fourth item

-----
Description list

- Rust _programming_ language:: A language empowering everyone
  to build reliable and efficient software.

-----
Table

|Syntax     |Description|
|-----------|-----------|
|Header     |Title      |
|Paragraph  |Text       |

-----
Image

[[https://www.rust-lang.org/static/images/rust-logo-blk.svg]]

-----
LaTeX

Render with \(\KaTeX\): $x+y$

$$
  f(\relax{x}) = \int_{-\infty}^\infty
    \hat{f}(\xi)\,e^{2 \pi i \xi x}
    \,d\xi
$$

\begin{align}
   a&=b+c \\
   d+e&=f
\end{align}

-----
Entity

\alpha\_   \rightarrow{}\_   \beta

-----
Subscript & superscript & line break

E= mc^2 \\
Fe_{_3_}O_4

"#;

const TEST_TABLE2: &'static str = r#"+++
toml = 1
+++
"#;

const TEST_TABLE3: &'static str = r#"---
yaml: 1
---
"#;


#[test]
pub fn test_table() {
    let reader = MarkdownParser::default();
    let _ = reader.load_str(TEST_TABLE1).unwrap();
}
