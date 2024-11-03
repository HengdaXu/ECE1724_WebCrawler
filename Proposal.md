# **Web Crawler with Data Analysis Proposal**

- Hengda Xu
- Zijin Liao

## **Table of Contents**
1. [Motivation](#motivation)
2. [Objective and Key Features](#objective-and-key-features)
3. [Tentative Plan](#tentative-plan)

## **Motivation**

In today, the rapid growth of information has made "data-driven decision-making"([`DDDM`](https://www.ibm.com/think/topics/data-driven-decision-making)) more important than ever. Whether mining large volumes of data for analysis, monitoring competitors' products, or aggregating news from various sources, the ability to collect data quickly and efficiently has become a core requirement. Thus, `Web Crawlers` become a key tool for fulfilling these needs.

On the other hand, `Rust`, as one of the newest and most popular programming languages, is known for its high performance and robust safety features. Rust’s efficiency makes it particularly suitable for developing data acquisition tools. However, unlike `Python`, which has a mature ecosystem of web scraping and analysis tools, Rust is still relatively young in this area. 

Although Rust has some effective libraries for web scraping, like `reqwest` and `scraper`, and data analysis libraries such as `polars`, tools for integrating data collecting and analysis are scarce. This often requires developers to call separate libraries for fetching and analyzing data, leading to extra steps. 

Ultimately, I aim to create a fully functional tool for filling the gap of Rust's ecosystem in integration of data crawling and analysis.

## **Objective and Key Features**

### **Objective**

With a background in big data, I understand that data acquisition and analysis are both important. My goal is to develop an `efficient`, `user-friendly` web crawler by Rust that not only enables `data crawling` but also provides `analysis capabilities`, for reducing the operations between collection and analysis.

### **Key Features**

- **Asynchronous crawling using stackless coroutines to speed up data collection**

    **Owners**: Hengda Xu, Zijin Liao

    `Tokio` library will be used in this project. It can achieves asynchronous operations by using stackless coroutines. When we are requesting the data from websites, the delay of responding time for a certain website can take a long time, and block the requesting for the other website. Therefore, we want to arrange the order of requesting and waiting using multithread to speed up the time for collecting the datas. `Aawait`, `async` and `future` libs will need in our implementation. When an `async` function encounters an `await` expression, it pauses execution of the current function and returns a `future` object to the scheduler. The scheduler can then arrange to execute other tasks without blocking the current thread while waiting for this task. Once the awaited operation completes, the `async` function resumes execution from the point where it paused at `await`.

    Meanwhile, `reqwest` will be used for sending Http request to the sever. `Reqwest` can convert `Rust types` to `JSON` when sending requests and parse `JSON` responses into Rust types using `Serde` instead of manually handling `JSON` parsing. This can make our work much more easier as we only need to extract important data from the map. `Reqwest` is fully compatible with Rust’s `async` and `await` syntax, making it ideal for applications that need to handle multiple requests concurrently without blocking the main thread.


- **Data parsing and extraction using HTML parsers**

    **Owners**: Hengda Xu, Zijin Liao

    For this feature, we can apply `Html5ever` or `Scraper` for HTML parsing. By comparing `Html5ever` with `Scraper`, we decide to use `Html5ever` instead of `Scraper`. `Html5ever` is a high-speed HTML parsing which can tansform the http document from a complicated `string` into a `Jason` hashmap. It's designed for Rust's performance and efficiency. Even though, `Scraper` is specifically designed for web scraping which are easier to setup, it is slower and might not be as good as `Html5ever` when dealing with complex `HTML` structures. Although `Html5ever` needs to integrate other libraries and write additional code to handle data extraction, we consider this as a good practice for us to be better in the code of Rust. 

- **Basic data analysis**

    **Owners**: Hengda Xu

    1. **Word Frequency Analysis**: In order to help users quickly identify keywords and popular topics, we will perform word frequency analysis on the crawled data. This involves labeling text as individual words and counting the frequency of each word. By highlighting frequently occurring terms, users can gain insight into the main topics related to the extracted words.

    2. **Sentiment Analysis (Optional)**: A simple sentiment analysis will be performed based on keywords to provide an initial assessment of the text's emotional tone (e.g., positive, negative, neutral). By analyzing common sentiment-indicating words, this feature aims to give users a general impression of the content’s mood, which can be useful for understanding public sentiment or tone within the data.

- **Text User Interface**

    **Owners**: Zijin Liao

    By using the `Cursive` or `Ratatui` library, we will develop an intuitive text-based user interface (`TUI`) for users to interactively view data analysis results. The `TUI` will display both the word frequency and sentiment analysis results and offer basic navigation and options. For instance, users can select different data sources to view specific results, refresh the data, and navigate through various sites’ analysis outputs.


## **Tentative Plan**