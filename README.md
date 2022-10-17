# Computer Notebook Format Proposal

Keywords: specification, computer science, organization, important

People: Shawn Garbett

## Introduction

A open sources means of storing and indexing notes in a simple manner on a computer that allows for future proofing that notebook for changing technologies. Ideally such a mechanism will rely on a format that allows for a variety of tools to assist, such as indexing, phone apps, etc. It must support web browsing. 

## Proposal

1. The notebook will be entirely contained within a single directory. Henceforth called `notebook` but could have any name a user wants. 
2. A note is stored in a sub-directory hierarchy YYYY/MM/DD.
    1. Any accompanying pictures or artifacts are stored in the same directory.
    2. These can be symbolic links to assets external to the `notebook` directory. *Note: The user must be careful that they have organized and intend to keep those external assets for the notebook to remain intact once symbolic links are utilized.*
3. A note is a modified [extended markdown](https://www.markdownguide.org).
    1. UTF-8 Extended markdown is used as the core specification.
    2. Superscripts are denoted by containment inside carets, e.g. `^superscript^`.
    3. Subscripts are denoted by single tildes, e.g. `~subscript~`. 
    4. Inline LaTeX math is denoted by containing dollar signs, e.g. `$f(x) = x^2$`.
    5. Breaking LaTeX math is denoted by double containing dollar signs, e.g. `$$f(x) = x^2$$`.
    6. The first Heading level 1 will be used as the document title.
        1. This heading must be the first markdown in the document.
    7. The first paragraph after the heading can contain indexing topics.
        1. A topic name is followed by a colon. E.g. `Keywords:`. 
        2. Following the colon is a comma separated list of topic entries. E.g. `Keywords: math, exponentials` or `People: John Lennon, Paul McCartney`. 
4. A process will exist that when run will create html views of the notebook. 
    1. It will output an `index.html` file to the `notebook` directory.
    2. It will be consist of sections.
        1. The process will accept a configuration parameter of ordered sections to include.
            1. It will default to `outline, calendar, keywords`.
        2. `outline` will be number of past days or notebooks entires outlines as links.
            1. The process will accept a configuration parameter for number of past days or number of notebook entries are desired.
            2. It will use the headers from the documents for an ordered outline by their header levels.
            3. The first level one header link will also include the date. 
        3. `calendar` will consist of a visual organization of links that expand a month into it's individual outline.
        4. `topics` will be an alphabetized multi-column listing of topic entries extracted from notes. 
            1. Each topic entry when clicked will dynamically expand to a series of links that consist of *title* and *date*. 
            2. Clicking another topic entry will close any prior opened topic entries.
    3. Any images the indexer creates will be in the directory `notebook/index`.
4. The system will not support live code blocks. I.e., This is not a quarto, Rmarkdown or Jupyter style notebook of code. This is intended to record and organize thoughts, ideas and artifacts of a daily journal.

## License

Copyright (c) 2022, Shawn P Garbett

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
