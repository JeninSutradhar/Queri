# Queri: DNS Resolver Server

[![Rust](https://img.shields.io/badge/rust-1.63+-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)

<p align="center">
  <img src="https://github.com/user-attachments/assets/a392f1df-1474-4556-b0d5-623427b663e5" alt="Queri Logo" width="300"/>
</p>

Queri is a simple yet powerful command-line DNS resolver built with Rust. It's designed to perform both iterative and recursive DNS lookups, providing you with detailed information about domain names. Queri supports various DNS record types, making it a versatile tool for network analysis and DNS exploration.

## ✨ Features

*   **Recursive DNS Resolution:** Recursively queries nameservers to find the IP addresses for domain names.
*   **Iterative Lookup Support:** Also supports iterative lookups for nameservers when needed.
*   **Support for A, AAAA, MX, NS, CNAME, SRV, and TXT Records:** Provides results for all essential record types.
*   **Caching Mechanism:** Improves performance by caching previous DNS queries.
*   **User-Friendly CLI:** An easy-to-use command-line interface.
*   **Colored Output:** Formatted output makes it easy to understand results.
*   **Dynamic Byte Packet Buffer:** Efficiently handles DNS message parsing and construction.

## ⚙️ How It Works

Queri's backend, upon receiving a query either from the server or the user, first uses a UDP socket to interact with DNS protocols, encoding the query into a binary DNS message. It then recursively queries nameservers, parsing each response by decoding the binary data to follow CNAME and NS records to reach an A, AAAA, MX, NS, CNAME, SRV, or TXT record, caching resolved results to improve efficiency and finally format the DNS responses to send to the user or client.


## 🛠️ Getting Started

### Prerequisites

*   **Rust:** You need to have the Rust toolchain installed. If you don't, visit [rustup.rs](https://rustup.rs/) for installation instructions.

### Building Queri

1.  **Clone the repository:**

    ```bash
    $ git clone https://github.com/JeninSutradhar/Queri
    $ cd Queri
    ```
2.  **Build the project:**

    ```bash
    $ cargo build --release
    ```
    This command compiles the project and creates the executable in the `target/release` folder.

### Running Queri

1.  **Run the executable:**

    ```bash
	$ cargo run
	```

    This will start Queri in interactive mode by default. You'll see a prompt asking for the domain and query type.
    ```bash
    Enter domain to resolve:
    ```
    and then for
    ```bash
    Enter query type (A, AAAA, MX, NS, CNAME, SRV, TXT, ALL):
    ```
   If any incoming query is received then Queri will act as a DNS server first.

## Example Output
```bash
$ Enter domain to resolve: www.yahoo.com
Enter query type (A, AAAA, MX, NS, CNAME, SRV, TXT, ALL): ALL

Attempting to resolve www.yahoo.com with type ALL

========================= DNS Resolution Results =========================
Query Type | Response Code | Record                                   | TTL  
--------------------------------------------------------------------
A          | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   
MX         | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   
NS         | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   
CNAME      | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   
SRV        | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   
TXT        | NOERROR      | www.yahoo.com - me-ycpi-cf-www.g06.yahoodns.net | 60   

========================= Resolution Complete =========================

Do you want to resolve another domain? (y/n): 
```

##  Using Queri as a DNS Server

You can send direct queries to Queri as a DNS server:

1. Start the Queri app by running `./target/release/queri`. This will make Queri act as a DNS server on port `8080`.
2. In a new terminal window use `dig` with `@127.0.0.1` to query Queri like:
    ```bash
    dig @127.0.0.1 -p 8080 google.com A
    ```

## 📦 Dependencies
```toml
[dependencies]
colored = "2.0"
```

## License

Queri is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contributing

Contributions are always welcome! Feel free to submit issues or pull requests for bugs, features, and improvements.

---
