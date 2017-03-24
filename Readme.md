Panduck document conversion tool
================================

Conversion tool by rust, inspired by [pandoc]().

## Formats

### Conversion

#### From (Text/Binary)

- [x] Markdown
- [x] HTML
- [ ] Jupyter(ipynb)
- [ ] Wiki
- [ ] Org-mode
- [ ] Rich Text Format(rtf)
- [ ] Office Word(docx)
- [ ] Office Excel(xls, xlsx, xlsm, xlsb, xla, xlam)
- [x] CSV/TSV
- [ ] Open Document Spread Sheets(ods)
- [ ] TeX
- [ ] BibTeX

#### Into (Rust + Text)

- [x] HTML
- [x] yew (+html_vdom)
- [x] sycamore (+html_dom)

### Highlights

- Text(txt)



## Developers



- Why not use `feature`

I used to use feature, but many formats are dependent on each other, and finally made a mess.

This makes me confused, so I canceled the feature division.