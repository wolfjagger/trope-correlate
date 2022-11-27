Architecture
===

Database
---

- scraped trope data
- cleaned scrapes
- (machine-learned) agent parameters
- summarized trope analysis

Scraper
---

- TVTropes web scraper

Cleaner
---

- Prepare scrape for machine learning analysis

Teacher
---

- Train agent
- Use collection of cleaned scrapes
- Control with hyperparameters

Lense
---

- Transform cleaned scrape into summarized data
- Uses agent to summarize

Display
---

- Transforms summarized data into wasm display
- Possibly also control training at this level

Website
---

- Show display data
- Include control of display and possibly of training
- Explain methods
- About devs
