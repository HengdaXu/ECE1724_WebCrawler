# Web Crawler with Data Analysis Proposal

- Hengda Xu
- Zijin Liao

## Table of Contents
1. [Motivation](#motivation)
2. [Objective and Key Features](#objective-and-key-features)
3. [Tentative Plan](#tentative-plan)

## Motivation

In today, the rapid growth of information has made "data-driven decision-making"([`DDDM`](https://www.ibm.com/think/topics/data-driven-decision-making)) more important than ever. Whether mining large volumes of data for analysis, monitoring competitors' products, or aggregating news from various sources, the ability to collect data quickly and efficiently has become a core requirement. Thus, `Web Crawlers` become a key tool for fulfilling these needs.

On the other hand, `Rust`, as one of the newest and most popular programming languages, is known for its high performance and robust safety features. Rust’s efficiency makes it particularly suitable for developing data acquisition tools. However, unlike `Python`, which has a mature ecosystem of web scraping and analysis tools, Rust is still relatively young in this area. 

Although Rust has some effective libraries for web scraping, like `reqwest` and `scraper`, and data analysis libraries such as `polars`, tools for integrating data collecting and analysis are scarce. This often requires developers to call separate libraries for fetching and analyzing data, leading to extra steps. 

Ultimately, I aim to create a fully functional tool for filling the gap of Rust's ecosystem in integration of data crawling and analysis.

## Objective and Key Features

### Objective

With a background in big data, I understand that data acquisition and analysis are both important. My goal is to develop an `efficient`, `user-friendly` web crawler by Rust that not only enables `data crawling` but also provides `analysis capabilities`, for reducing the operations between collection and analysis.

### Key Features

1. Asynchronous crawling using stackless coroutines to speed up data collection

`Tokio` library will be used in this project. It can achieves asynchronous operations by using stackless coroutines. When we are requesting the data from websites, the delay of responding time for a certain website can take a long time, and block the requesting for the other website. Therefore, we want to arrange the order of requesting and waiting using multithread to speed up the time for collecting the datas. `Aawait`, `async` and `future` libs will need in our implementation. When an `async` function encounters an `await` expression, it pauses execution of the current function and returns a `future` object to the scheduler. The scheduler can then arrange to execute other tasks without blocking the current thread while waiting for this task. Once the awaited operation completes, the `async` function resumes execution from the point where it paused at `await`.

## Tentative Plan