"""A quick script to help download TvTropes pages with curl."""

import subprocess
from os import makedirs
from os.path import join
from time import sleep


CURL_COMMAND: list[str] = [
    "curl",
    "--connect-timeout",
    "60",
    "-G",
    "-d",
    "",
    "-d",
    "",
    "-d",
    "",
    "https://tvtropes.org/pmwiki/pagelist_having_pagetype_in_namespace.php",
]

NAMESPACE_PREFIX = "n="
NAMESPACE_INDEX = 5
PAGE_TYPE_PREFIX = "t="
PAGE_TYPE_INDEX = 7
PAGE_PREFIX = "page="
PAGE_INDEX = 9


def main() -> None:
    """Download all the pages."""
    namespace = "Main"
    page_type = "trope"

    path_dir: str = join("test_data", namespace + "_" + page_type, "")
    makedirs(path_dir, exist_ok=True)

    curl_command = CURL_COMMAND.copy()
    curl_command[NAMESPACE_INDEX] = NAMESPACE_PREFIX + namespace
    curl_command[PAGE_TYPE_INDEX] = PAGE_TYPE_PREFIX + page_type

    page = 1

    while page < 59:
        curl_command[PAGE_INDEX] = PAGE_PREFIX + str(page)
        filename = path_dir + "page" + str(page) + ".html"
        print(curl_command)
        html = subprocess.run(curl_command, check=False, capture_output=True, text=True)
        with open(filename, "w") as file:
            file.write(html.stdout)
        page += 1
        sleep(0.5)


if __name__ == "__main__":
    main()
