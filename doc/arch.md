Architecture
===

Database (postgre)
---

Stores:
- scraped trope data
- cleaned scrapes
- (machine-learned) agent parameters
- summarized trope analysis

Normalized where possible, while keeping performance in mind.

Scraper (rust)
---

TVTropes web scraper.
Interactive via command line.
Scrape tropes and media, including different "tags" like YMMV.
Store scrape in database cleanly.

Cleaner (rust or python)
---

Prepare scrape for machine learning analysis.
Pull from & push to db.
Timestamped and possibly versioned.

Teacher (python)
---

Train agent.
Use collection of cleaned scrapes.
Control with hyperparameters.
Start as simple as possible: predict a number.
Save parameters for agent in db with metadata (e.g. hyperparameters).

Lense (rust or python)
---

Transform cleaned scrape into summarized data.
Use agent to summarize.

Display (rust wasm)
---

Transforms summarized data into wasm display.
Possibly also control training or agent selection at this level.

Website (rust yew, possibly typescript)
---

Show display data
Include control of display and possibly of training
Explain methods
About devs
