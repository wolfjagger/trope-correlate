"""A quick script to help download TvTropes pages with curl."""

import argparse
from bs4 import BeautifulSoup
from urllib.request import urlopen
from functools import partial
from os import makedirs
from os.path import join
from time import sleep


NAMESPACE_PREFIX = "n="
PAGETYPE_PREFIX = "t="
PAGENUM_PREFIX = "page="
TVTROPES_SEARCH_PAGE = (
    "https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php"
)


def main() -> None:
    """Download all the pages."""
    args = create_arg_parser().parse_args()

    path_dir: str = join("test_data", args.namespace + "_" + args.pagetype, "")
    makedirs(path_dir, exist_ok=True)

    page = 1

    page_url = partial(create_url, args.namespace, args.pagetype)

    while page <= args.max_pages:
        filename = path_dir + "page" + str(page) + ".html"
        url = page_url(page)
        if url.startswith("http"):
            with urlopen(url) as site, open(filename, "w") as file:  # nosec
                html = site.read().decode()
                soup = BeautifulSoup(html, "html.parser")
                file.write(html)
        page += 1
        sleep(1)


def create_url(namespace: str, pagetype: str, page: int) -> str:
    """Define the url string from the query arguments."""
    return (
        TVTROPES_SEARCH_PAGE
        + "?"
        + NAMESPACE_PREFIX
        + namespace
        + "&"
        + PAGETYPE_PREFIX
        + pagetype
        + "&"
        + PAGENUM_PREFIX
        + str(page)
    )


def create_arg_parser() -> argparse.ArgumentParser:
    """Create an object to parse the arguments."""
    parser = argparse.ArgumentParser(
        prog="trope-downloader",
        description="Downloads index pages in bulk from tvtropes.",
    )
    parser.add_argument("-n", "--namespace", default="Main")
    parser.add_argument("-t", "--pagetype", default="trope")
    parser.add_argument("--max_pages", default=2)
    return parser


if __name__ == "__main__":
    main()
