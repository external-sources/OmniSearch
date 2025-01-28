# OmniSearch

Welcome to **OmniSearch**, a powerful and comprehensive search tool brought to you by the **My Privacy DNS** project! Our mission is to enhance your search experience by aggregating results from multiple sources, allowing you to find information seamlessly across various platforms, applications, and repositories. With OmniSearch, we aim to streamline information retrieval and boost productivity, especially in enterprise environments.

## Project Overview

OmniSearch is designed to import external sources such as hosts, JSON files, plain text files, and MariaDB databases. We load this data into a flexible database format, including JSON, CSV, or MariaDB, and provide robust sorting and searching capabilities. Our tool keeps track of the sources for domain and URI records, ensuring that you have access to reliable and organized information.

### Key Features

- **Data Import**: Easily import data from various external sources, including hosts, JSON, plain text files, and MariaDB.
- **Flexible Database Loading**: Load data into JSON, CSV, or MariaDB formats for easy access and management.
- **Advanced Sorting**: Sort results by domain, URI, and category to find exactly what you need.
- **Source Tracking**: Keep track of the sources for domain and URI records for better transparency and reliability.
- **Search Command**: Execute search commands that sort and list results based on your preferences.
    - Sort output by source, then records.
    - Clean format output without sources.
    - Output records from non-DNS sources like EasyList, uBlock, and Adguard.
    - Access records from the My Privacy DNS project Matrix.
    - Extract MyPDNS RPZ records directly from MariaDB.

### Record Management

We provide tools to manage Matrix records, allowing you to add, delete, or alter records through:

- **PowerDNS Auth Server's API**: Interact with the PowerDNS API for authoritative zone management. [API Documentation](https://docs.powerdns.com/authoritative/http-api/zone.html)
- **Source File Alteration**: Modify source files within the Matrix source directory for customized data management.

### Availability Testing

Before committing any changes, we incorporate **PyFunceble** to test the availability of domains, ensuring that your data remains accurate and up-to-date.

### Future Developments

We are actively working on integrating a web crawler that will categorize sites and scripts, primarily focusing on tracking and adult sites. This feature will further enhance our search capabilities and provide users with even more comprehensive results.

## Get Started

To get started with OmniSearch, visit our project page at [My Privacy DNS](http://www.mypdns.org) and explore the documentation for installation and usage instructions.

Join us in revolutionizing the way you search for information! With OmniSearch, we empower you to find what you need, when you need it, all in one place.
