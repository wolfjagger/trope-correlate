Trope Download
===

Download tropes from tvtropes.org.

Header file
---

To run any of these, you need header information in a local `trope-download/.header` file.
First, load tvtropes.org, e.g. https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php?n=Main&t=trope&page=1.
Open the browser console (ctrl+shift+i).
Find the "request header" (in firefox, the "Network tab", first entry, toggle "Raw").
Copy the header from "User-Agent" onward into a `.../trope-correlate/trope-download/.header` file.

Downloading
---

### Download trope page

You can download tropes with specified name and url with the subcommand `trope-page`.
For example, to download the "AbandonedPlayground" trope, run:

`cargo run trope-page -n AbandonedPlayground -u http://tvtropes.org/pmwiki/pmwiki.php/Main/AbandonedPlayground`

This will download the page to
 `trope-correlate/test_data/trope_page/{name}.html.br`.
It will skip files previously downloaded unless `--force` is specified.
This will really download any `{url}` and use the `{name}` to set the output file.

See command help for more info.

### Download trope pagelist

You can download pagelists from
 https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php
 with the subcommand `pagelist`.
For example, to download pages 1 to 58 from the "Main" namespace with pagetype "trope"
 (the main list of tropes), run:

`cargo run pagelist -b 1 -e 58 -n main -p trope`

This will download each page in turn to
 `trope-correlate/test_data/{namespace}/{pagetype}/page{num}.html.br`.
It will skip files previously downloaded unless `--force` is specified.
Between each download, it will sleep for some number of seconds, min `1` and default `5`.

See command help for more configuration.

### Download tropelist

You can download tropelists from scraped pagelists with the subcommand `tropelist`.
For example, to download the first 50 tropes from a tropelist at
 `../test_data/tropes.csv`, run:

`cargo run tropelist -i ../test_data/tropes.csv -b 1 -e 50`

This will download each page in turn to
 `trope-correlate/test_data/trope_page/{name}.html.br`, where the name is given in the tropelist.
It will skip files previously downloaded unless `--force` is specified.
Between each download, it will sleep for some number of seconds, min `1` and default `5`.

NOTE: There is no randomization by default, but it is recommended to do so.
You may specify `-r {seed}` to download the pages randomly with the given seed.
The seed may be used for reproducibility and to resume previous downloads.
For example, if you ran `cargo run tropelist -i ../test_data/tropes.csv -b 1 -e 500 -r 43`
 and you cancelled at trope 300, you could rerun the same command and it would skip the first
 300 tropes and resume while keeping the same randomization.

See command help for more configuration.


Notes
---

By default, you will download Brotli-compressed files, `*.html.br`.
The sibling trope-scrape project can decrypt these on demand, and this default will save space.
If you want to disable this and download the raw html files, you can specify `--encrypted false`
 for most of these subcommands.
