"""A quick script to help download TvTropes pages with curl."""

import argparse
import subprocess
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

    page_command = partial(create_curl_command, args.namespace, args.pagetype)

    while page <= args.max_pages:
        curl_command = page_command(str(page))
        filename = path_dir + "page" + str(page) + ".html"
        print(curl_command)
        html = subprocess.run(curl_command, check=False, capture_output=True, text=True)
        with open(filename, "w") as file:
            file.write(html.stdout)
        page += 1
        sleep(1)


def create_curl_command(namespace: str, pagetype: str, page: str) -> list[str]:
    """Create a command for curl to download a tvtropes page."""
    command: list[str] = [
        "curl",
        "--connect-timeout",
        "60",
        "-G",
        "-d",
        NAMESPACE_PREFIX + namespace,
        "-d",
        PAGETYPE_PREFIX + pagetype,
        "-d",
        PAGENUM_PREFIX + page,
        TVTROPES_SEARCH_PAGE,
    ]
    return command


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
